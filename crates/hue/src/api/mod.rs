mod device;
mod entertainment;
mod entertainment_config;
mod ext;
mod grouped_light;
mod light;
mod resource;
mod room;
mod scene;
mod stream;
mod stubs;
mod update;
mod zigbee_device_discovery;

pub use device::{Device, DeviceArchetype, DeviceProductData, DeviceUpdate, Identify};
pub use entertainment::{Entertainment, EntertainmentSegment, EntertainmentSegments};
pub use entertainment_config::{
    EntertainmentConfiguration, EntertainmentConfigurationAction,
    EntertainmentConfigurationChannels, EntertainmentConfigurationLocations,
    EntertainmentConfigurationLocationsNew, EntertainmentConfigurationLocationsUpdate,
    EntertainmentConfigurationMetadata, EntertainmentConfigurationNew,
    EntertainmentConfigurationServiceLocations, EntertainmentConfigurationServiceLocationsNew,
    EntertainmentConfigurationServiceLocationsUpdate, EntertainmentConfigurationStatus,
    EntertainmentConfigurationStreamMembers, EntertainmentConfigurationStreamProxy,
    EntertainmentConfigurationStreamProxyMode, EntertainmentConfigurationStreamProxyUpdate,
    EntertainmentConfigurationType, EntertainmentConfigurationUpdate, Position,
};
pub use ext::ResourceExt;
pub use grouped_light::{GroupedLight, GroupedLightUpdate};
pub use light::{
    ColorGamut, ColorTemperature, ColorTemperatureUpdate, ColorUpdate, Delta, Dimming,
    DimmingUpdate, GamutType, Light, LightAlert, LightColor, LightDynamics, LightDynamicsStatus,
    LightEffect, LightEffectActionUpdate, LightEffectParameters, LightEffectStatus,
    LightEffectValues, LightEffects, LightEffectsV2, LightEffectsV2Update, LightFunction,
    LightGradient, LightGradientMode, LightGradientPoint, LightGradientUpdate, LightMetadata,
    LightMode, LightPowerup, LightPowerupColor, LightPowerupDimming, LightPowerupOn,
    LightPowerupPreset, LightProductData, LightSignal, LightSignaling, LightTimedEffects,
    LightUpdate, MirekSchema, On,
};
pub use resource::{RType, ResourceLink, ResourceRecord};
pub use room::{Room, RoomArchetype, RoomMetadata, RoomMetadataUpdate, RoomUpdate};
pub use scene::{
    Scene, SceneAction, SceneActionElement, SceneActive, SceneMetadata, SceneRecall, SceneStatus,
    SceneStatusEnum, SceneUpdate,
};
pub use stream::HueStreamKey;
pub use stubs::{
    BehaviorInstance, BehaviorInstanceMetadata, BehaviorScript, Bridge, BridgeHome, Button,
    ButtonData, ButtonMetadata, ButtonReport, CameraMotion, Contact, DevicePower,
    DeviceSoftwareUpdate, DollarRef, GeofenceClient, Geolocation, GroupedLightLevel, GroupedMotion,
    Homekit, LightLevel, Matter, MatterFabric, Metadata, MetadataUpdate, Motion, PrivateGroup,
    PublicImage, RelativeRotary, ServiceGroup, SmartScene, Tamper, Taurus, Temperature, TimeZone,
    ZgpConnectivity, ZigbeeConnectivity, ZigbeeConnectivityStatus, Zone,
};
pub use update::Update;
pub use zigbee_device_discovery::{
    ZigbeeDeviceDiscovery, ZigbeeDeviceDiscoveryAction, ZigbeeDeviceDiscoveryInstallCode,
    ZigbeeDeviceDiscoveryStatus, ZigbeeDeviceDiscoveryUpdate, ZigbeeDeviceDiscoveryUpdateAction,
    ZigbeeDeviceDiscoveryUpdateActionType,
};

use std::fmt::Debug;

use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::error::{HueError, HueResult};
use crate::legacy_api::ApiLightStateUpdate;

#[derive(Debug, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Stub;

impl Serialize for Stub {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_map(None)?.end()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Resource {
    BehaviorInstance(BehaviorInstance),
    BehaviorScript(BehaviorScript),
    Bridge(Bridge),
    BridgeHome(BridgeHome),
    Button(Button),
    Device(Device),
    DevicePower(DevicePower),
    DeviceSoftwareUpdate(DeviceSoftwareUpdate),
    Entertainment(Entertainment),
    EntertainmentConfiguration(EntertainmentConfiguration),
    GeofenceClient(GeofenceClient),
    Geolocation(Geolocation),
    GroupedLight(GroupedLight),
    GroupedLightLevel(GroupedLightLevel),
    GroupedMotion(GroupedMotion),
    Homekit(Homekit),
    Light(Light),
    LightLevel(LightLevel),
    Matter(Matter),
    Motion(Motion),
    PrivateGroup(PrivateGroup),
    PublicImage(PublicImage),
    RelativeRotary(RelativeRotary),
    Room(Room),
    Scene(Scene),
    SmartScene(SmartScene),
    #[serde(rename = "taurus_7455")]
    Taurus(Taurus),
    Temperature(Temperature),
    ZigbeeConnectivity(ZigbeeConnectivity),
    ZigbeeDeviceDiscovery(ZigbeeDeviceDiscovery),
    Zone(Zone),

    /* Unmapped variants */
    CameraMotion(CameraMotion),
    Contact(Contact),
    MatterFabric(MatterFabric),
    ServiceGroup(ServiceGroup),
    Tamper(Tamper),
    ZgpConnectivity(ZgpConnectivity),
}

#[macro_export]
macro_rules! resource_conversion_impl {
    ( $name:ident ) => {
        impl<'a> TryFrom<&'a mut Resource> for &'a mut $name {
            type Error = HueError;

            fn try_from(value: &'a mut Resource) -> Result<Self, Self::Error> {
                if let Resource::$name(obj) = value {
                    Ok(obj)
                } else {
                    Err(HueError::WrongType(RType::$name, value.rtype()))
                }
            }
        }

        impl<'a> TryFrom<&'a Resource> for &'a $name {
            type Error = HueError;

            fn try_from(value: &'a Resource) -> Result<Self, Self::Error> {
                if let Resource::$name(obj) = value {
                    Ok(obj)
                } else {
                    Err(HueError::WrongType(RType::$name, value.rtype()))
                }
            }
        }

        impl TryFrom<Resource> for $name {
            type Error = HueError;

            fn try_from(value: Resource) -> Result<Self, Self::Error> {
                if let Resource::$name(obj) = value {
                    Ok(obj)
                } else {
                    Err(HueError::WrongType(RType::$name, value.rtype()))
                }
            }
        }

        impl From<$name> for Resource {
            fn from(value: $name) -> Self {
                Resource::$name(value)
            }
        }
    };
}

resource_conversion_impl!(BehaviorInstance);
resource_conversion_impl!(BehaviorScript);
resource_conversion_impl!(Bridge);
resource_conversion_impl!(BridgeHome);
resource_conversion_impl!(Button);
resource_conversion_impl!(Device);
resource_conversion_impl!(DevicePower);
resource_conversion_impl!(DeviceSoftwareUpdate);
resource_conversion_impl!(Entertainment);
resource_conversion_impl!(EntertainmentConfiguration);
resource_conversion_impl!(GeofenceClient);
resource_conversion_impl!(Geolocation);
resource_conversion_impl!(GroupedLight);
resource_conversion_impl!(GroupedLightLevel);
resource_conversion_impl!(GroupedMotion);
resource_conversion_impl!(Homekit);
resource_conversion_impl!(Light);
resource_conversion_impl!(LightLevel);
resource_conversion_impl!(Matter);
resource_conversion_impl!(Motion);
resource_conversion_impl!(PrivateGroup);
resource_conversion_impl!(PublicImage);
resource_conversion_impl!(RelativeRotary);
resource_conversion_impl!(Room);
resource_conversion_impl!(Scene);
resource_conversion_impl!(SmartScene);
resource_conversion_impl!(Taurus);
resource_conversion_impl!(Temperature);
resource_conversion_impl!(ZigbeeConnectivity);
resource_conversion_impl!(ZigbeeDeviceDiscovery);
resource_conversion_impl!(Zone);

#[derive(Clone, Debug, Serialize)]
pub struct V1Reply<'a> {
    prefix: String,
    success: Vec<(&'a str, Value)>,
}

impl<'a> V1Reply<'a> {
    #[must_use]
    pub const fn new(prefix: String) -> Self {
        Self {
            prefix,
            success: vec![],
        }
    }

    #[must_use]
    pub fn for_light(id: u32, path: &str) -> Self {
        Self::new(format!("/lights/{id}/{path}"))
    }

    #[must_use]
    pub fn for_group_path(id: u32, path: &str) -> Self {
        Self::new(format!("/groups/{id}/{path}"))
    }

    #[must_use]
    pub fn for_group(id: u32) -> Self {
        Self::new(format!("/groups/{id}"))
    }

    pub fn with_light_state_update(self, upd: &ApiLightStateUpdate) -> HueResult<Self> {
        self.add_option("on", upd.on)?
            .add_option("bri", upd.bri)?
            .add_option("xy", upd.xy)?
            .add_option("ct", upd.ct)
    }

    pub fn add<T: Serialize>(mut self, name: &'a str, value: T) -> HueResult<Self> {
        self.success.push((name, serde_json::to_value(value)?));
        Ok(self)
    }

    pub fn add_option<T: Serialize>(mut self, name: &'a str, value: Option<T>) -> HueResult<Self> {
        if let Some(val) = value {
            self.success.push((name, serde_json::to_value(val)?));
        }
        Ok(self)
    }

    #[must_use]
    pub fn json(self) -> Value {
        let mut json = vec![];
        let prefix = self.prefix;
        for (name, value) in self.success {
            json.push(json!({"success": {format!("{prefix}/{name}"): value}}));
        }
        json!(json)
    }
}
