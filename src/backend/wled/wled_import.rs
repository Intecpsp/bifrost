use maplit::btreeset;

use hue::api::{
    ColorTemperature, DeviceArchetype, DeviceProductData, Dimming, Entertainment,
    EntertainmentSegment, EntertainmentSegments, Light, LightColor, LightEffects, LightEffectsV2,
    LightMetadata, MirekSchema, RType, Resource, Stub, Taurus, ZigbeeConnectivity,
};
use hue::xy::XY;
use wled::{Color, SegCap, StateSeg, WledInfo};

use crate::backend::wled::WledBackend;
use crate::error::ApiResult;

impl WledBackend {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub async fn add_light(
        &mut self,
        index: u8,
        state: &StateSeg,
        features: SegCap,
        info: &WledInfo,
    ) -> ApiResult<()> {
        let name = &info.name;

        let link_device = self.device_link(index);

        let link_light = RType::Light.deterministic(link_device);
        let link_enttm = RType::Entertainment.deterministic(link_device);
        let link_taurus = RType::Taurus.deterministic(link_device);
        let link_zigcon = RType::ZigbeeConnectivity.deterministic(link_device);

        let product_data = DeviceProductData {
            model_id: self.server.url.to_string(),
            manufacturer_name: info.brand.clone(),
            product_name: info.product.clone(),
            product_archetype: DeviceArchetype::HueLightstrip,
            certified: false,
            software_version: info.ver.clone(),
            hardware_platform_type: None,
        };
        let metadata = LightMetadata::new(product_data.product_archetype.clone(), name);

        let dev = hue::api::Device {
            product_data,
            metadata: metadata.clone().into(),
            services: btreeset![link_zigcon, link_light, link_enttm, link_taurus],
            identify: Some(Stub),
            usertest: None,
        };

        let mut light = Light::new(link_device, metadata);

        light.dimming = Some(Dimming {
            brightness: f64::from(state.bri) / 2.55,
            min_dim_level: Some(1.0 / 255.0),
        });
        log::trace!("Detected dimming: {:?}", &light.dimming);

        light.color_temperature = None;

        if features.contains(SegCap::CCT) {
            light.color_temperature = Some(ColorTemperature {
                mirek: Some(((f64::from(state.cct) / 255.0).mul_add(424.0, 100.0)) as u16),
                mirek_valid: false,
                mirek_schema: MirekSchema {
                    mirek_minimum: 100,
                    mirek_maximum: 525,
                },
            });
            log::trace!("Detected color temperature: {:?}", &light.color_temperature);
        }

        if features.contains(SegCap::RGB) {
            let fc = state.col.primary;
            match fc {
                Color::Rgb([r, g, b]) | Color::Rgbw([r, g, b, _]) => {
                    let xy = XY::from_rgb(r, g, b).0;
                    light.color = Some(LightColor::new(xy));
                    log::trace!("Detected color: {:?}", &light.color);
                }
                Color::None([]) => {}
            }
            log::trace!("Detected color: {:?}", &light.color);
        }

        light.effects = Some(LightEffects::all());
        light.effects_v2 = Some(LightEffectsV2::all());

        /* light.gradient = gradient.and_then(ExtractLightGradient::extract_from_expose); */
        /* log::trace!("Detected gradient support: {:?}", &light.gradient); */

        self.map.insert(index, link_light);
        self.rmap.insert(link_device, index);
        self.rmap.insert(link_light, index);

        let segments = EntertainmentSegments {
            configurable: false,
            max_segments: 10,
            segments: (0..7)
                .map(|x| EntertainmentSegment {
                    start: x,
                    length: 1,
                })
                .collect(),
        };

        // FIXME: This should be feature-detected, not always enabled
        let enttm = Entertainment {
            equalizer: true,
            owner: link_device,
            proxy: true,
            renderer: true,
            max_streams: None,
            renderer_reference: Some(link_light),
            segments: Some(segments),
        };

        // FIXME: The Taurus objects are seen on Hue Entertainment devices on a
        // real hue bridge, but nobody knows what it does. Some clients seem to
        // want them present, though.
        let taurus = Taurus {
            capabilities: vec![
                "sensor".to_string(),
                "collector".to_string(),
                "sync".to_string(),
            ],
            owner: link_device,
        };

        let zigcon =
            ZigbeeConnectivity::from_owner_and_mac(link_device, format!("{}-{}", info.mac, index));

        let mut res = self.state.lock().await;
        res.add(&link_device, Resource::Device(dev))?;
        res.add(&link_light, Resource::Light(light))?;
        res.add(&link_enttm, Resource::Entertainment(enttm))?;
        res.add(&link_taurus, Resource::Taurus(taurus))?;
        res.add(&link_zigcon, Resource::ZigbeeConnectivity(zigcon))?;
        drop(res);

        Ok(())
    }
}
