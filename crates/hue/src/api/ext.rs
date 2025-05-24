use crate::api::{
    BehaviorInstance, BehaviorScript, Bridge, BridgeHome, Button, CameraMotion, Contact, Device,
    DevicePower, DeviceSoftwareUpdate, Entertainment, EntertainmentConfiguration, GeofenceClient,
    Geolocation, GroupedLight, GroupedLightLevel, GroupedMotion, Homekit, Light, LightLevel,
    Matter, MatterFabric, Motion, PrivateGroup, PublicImage, RType, RelativeRotary, ResourceLink,
    Room, Scene, ServiceGroup, SmartScene, Tamper, Taurus, Temperature, ZgpConnectivity,
    ZigbeeConnectivity, ZigbeeDeviceDiscovery, Zone,
};

macro_rules! impl_resource_dispatch {
    ($func:ident) => {
        impl_resource_dispatch!($func,);
    };

    ($func:ident, $args:expr) => {
        match self {
            Self::AuthV1(obj) => obj.$func($expr),
            Self::BehaviorInstance(obj) => obj.$func($expr),
            Self::BehaviorScript(obj) => obj.$func($expr),
            Self::Bridge(obj) => obj.$func($expr),
            Self::BridgeHome(obj) => obj.$func($expr),
            Self::Button(obj) => obj.$func($expr),
            Self::CameraMotion(obj) => obj.$func($expr),
            Self::Contact(obj) => obj.$func($expr),
            Self::Device(obj) => obj.$func($expr),
            Self::DevicePower(obj) => obj.$func($expr),
            Self::DeviceSoftwareUpdate(obj) => obj.$func($expr),
            Self::Entertainment(obj) => obj.$func($expr),
            Self::EntertainmentConfiguration(obj) => obj.$func($expr),
            Self::GeofenceClient(obj) => obj.$func($expr),
            Self::Geolocation(obj) => obj.$func($expr),
            Self::GroupedLight(obj) => obj.$func($expr),
            Self::GroupedLightLevel(obj) => obj.$func($expr),
            Self::GroupedMotion(obj) => obj.$func($expr),
            Self::Homekit(obj) => obj.$func($expr),
            Self::Light(obj) => obj.$func($expr),
            Self::LightLevel(obj) => obj.$func($expr),
            Self::Matter(obj) => obj.$func($expr),
            Self::MatterFabric(obj) => obj.$func($expr),
            Self::Motion(obj) => obj.$func($expr),
            Self::PrivateGroup(obj) => obj.$func($expr),
            Self::PublicImage(obj) => obj.$func($expr),
            Self::RelativeRotary(obj) => obj.$func($expr),
            Self::Room(obj) => obj.$func($expr),
            Self::Scene(obj) => obj.$func($expr),
            Self::ServiceGroup(obj) => obj.$func($expr),
            Self::SmartScene(obj) => obj.$func($expr),
            Self::Tamper(obj) => obj.$func($expr),
            Self::Taurus(obj) => obj.$func($expr),
            Self::Temperature(obj) => obj.$func($expr),
            Self::ZgpConnectivity(obj) => obj.$func($expr),
            Self::ZigbeeConnectivity(obj) => obj.$func($expr),
            Self::ZigbeeDeviceDiscovery(obj) => obj.$func($expr),
            Self::Zone(obj) => obj.$func($expr),
        }
    };
}

pub trait ResourceExt {
    fn rtype(&self) -> RType;

    fn owner(&self) -> Option<ResourceLink>;

    fn delete_link(&mut self, _rlink: &ResourceLink) {}
}

/* impl ResourceExt for AuthV1 {} */
impl ResourceExt for BehaviorInstance {
    fn rtype(&self) -> RType {
        RType::BehaviorInstance
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for BehaviorScript {
    fn rtype(&self) -> RType {
        RType::BehaviorScript
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for Bridge {
    fn rtype(&self) -> RType {
        RType::Bridge
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for BridgeHome {
    fn rtype(&self) -> RType {
        RType::BridgeHome
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for Button {
    fn rtype(&self) -> RType {
        RType::Button
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for Device {
    fn rtype(&self) -> RType {
        RType::Device
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for DevicePower {
    fn rtype(&self) -> RType {
        RType::DevicePower
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for DeviceSoftwareUpdate {
    fn rtype(&self) -> RType {
        RType::DeviceSoftwareUpdate
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for Entertainment {
    fn rtype(&self) -> RType {
        RType::Entertainment
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for EntertainmentConfiguration {
    fn rtype(&self) -> RType {
        RType::EntertainmentConfiguration
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for GeofenceClient {
    fn rtype(&self) -> RType {
        RType::GeofenceClient
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for Geolocation {
    fn rtype(&self) -> RType {
        RType::Geolocation
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for GroupedLight {
    fn rtype(&self) -> RType {
        RType::GroupedLight
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for GroupedLightLevel {
    fn rtype(&self) -> RType {
        RType::GroupedLightLevel
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for GroupedMotion {
    fn rtype(&self) -> RType {
        RType::GroupedMotion
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for Homekit {
    fn rtype(&self) -> RType {
        RType::Homekit
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for Light {
    fn rtype(&self) -> RType {
        RType::Light
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for LightLevel {
    fn rtype(&self) -> RType {
        RType::LightLevel
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for Matter {
    fn rtype(&self) -> RType {
        RType::Matter
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for Motion {
    fn rtype(&self) -> RType {
        RType::Motion
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for PrivateGroup {
    fn rtype(&self) -> RType {
        RType::PrivateGroup
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for PublicImage {
    fn rtype(&self) -> RType {
        RType::PublicImage
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for RelativeRotary {
    fn rtype(&self) -> RType {
        RType::RelativeRotary
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for Room {
    fn rtype(&self) -> RType {
        RType::Room
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for Scene {
    fn rtype(&self) -> RType {
        RType::Scene
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for SmartScene {
    fn rtype(&self) -> RType {
        RType::SmartScene
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for Taurus {
    fn rtype(&self) -> RType {
        RType::Taurus
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for Temperature {
    fn rtype(&self) -> RType {
        RType::Temperature
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for ZigbeeConnectivity {
    fn rtype(&self) -> RType {
        RType::ZigbeeConnectivity
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for ZigbeeDeviceDiscovery {
    fn rtype(&self) -> RType {
        RType::ZigbeeDeviceDiscovery
    }

    fn owner(&self) -> Option<ResourceLink> {
        Some(self.owner)
    }
}

impl ResourceExt for Zone {
    fn rtype(&self) -> RType {
        RType::Zone
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for CameraMotion {
    fn rtype(&self) -> RType {
        RType::CameraMotion
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for Contact {
    fn rtype(&self) -> RType {
        RType::Contact
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for MatterFabric {
    fn rtype(&self) -> RType {
        RType::MatterFabric
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for ServiceGroup {
    fn rtype(&self) -> RType {
        RType::ServiceGroup
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for Tamper {
    fn rtype(&self) -> RType {
        RType::Tamper
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}

impl ResourceExt for ZgpConnectivity {
    fn rtype(&self) -> RType {
        RType::ZgpConnectivity
    }

    fn owner(&self) -> Option<ResourceLink> {
        None
    }
}
