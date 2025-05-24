use crate::api::{
    BehaviorInstance, BehaviorScript, Bridge, BridgeHome, Button, CameraMotion, Contact, Device,
    DevicePower, DeviceSoftwareUpdate, Entertainment, EntertainmentConfiguration, GeofenceClient,
    Geolocation, GroupedLight, GroupedLightLevel, GroupedMotion, Homekit, Light, LightLevel,
    Matter, MatterFabric, Motion, PrivateGroup, PublicImage, RType, RelativeRotary, Resource,
    ResourceLink, Room, Scene, ServiceGroup, SmartScene, Tamper, Taurus, Temperature,
    ZgpConnectivity, ZigbeeConnectivity, ZigbeeDeviceDiscovery, Zone,
};

macro_rules! impl_resource_dispatch {
    ($val:expr, $func:ident $(, $args:expr)?) => {
        match $val {
            Self::BehaviorInstance(obj) => obj.$func($($args)?),
            Self::BehaviorScript(obj) => obj.$func($($args)?),
            Self::Bridge(obj) => obj.$func($($args)?),
            Self::BridgeHome(obj) => obj.$func($($args)?),
            Self::Button(obj) => obj.$func($($args)?),
            Self::CameraMotion(obj) => obj.$func($($args)?),
            Self::Contact(obj) => obj.$func($($args)?),
            Self::Device(obj) => obj.$func($($args)?),
            Self::DevicePower(obj) => obj.$func($($args)?),
            Self::DeviceSoftwareUpdate(obj) => obj.$func($($args)?),
            Self::Entertainment(obj) => obj.$func($($args)?),
            Self::EntertainmentConfiguration(obj) => obj.$func($($args)?),
            Self::GeofenceClient(obj) => obj.$func($($args)?),
            Self::Geolocation(obj) => obj.$func($($args)?),
            Self::GroupedLight(obj) => obj.$func($($args)?),
            Self::GroupedLightLevel(obj) => obj.$func($($args)?),
            Self::GroupedMotion(obj) => obj.$func($($args)?),
            Self::Homekit(obj) => obj.$func($($args)?),
            Self::Light(obj) => obj.$func($($args)?),
            Self::LightLevel(obj) => obj.$func($($args)?),
            Self::Matter(obj) => obj.$func($($args)?),
            Self::MatterFabric(obj) => obj.$func($($args)?),
            Self::Motion(obj) => obj.$func($($args)?),
            Self::PrivateGroup(obj) => obj.$func($($args)?),
            Self::PublicImage(obj) => obj.$func($($args)?),
            Self::RelativeRotary(obj) => obj.$func($($args)?),
            Self::Room(obj) => obj.$func($($args)?),
            Self::Scene(obj) => obj.$func($($args)?),
            Self::ServiceGroup(obj) => obj.$func($($args)?),
            Self::SmartScene(obj) => obj.$func($($args)?),
            Self::Tamper(obj) => obj.$func($($args)?),
            Self::Taurus(obj) => obj.$func($($args)?),
            Self::Temperature(obj) => obj.$func($($args)?),
            Self::ZgpConnectivity(obj) => obj.$func($($args)?),
            Self::ZigbeeConnectivity(obj) => obj.$func($($args)?),
            Self::ZigbeeDeviceDiscovery(obj) => obj.$func($($args)?),
            Self::Zone(obj) => obj.$func($($args)?),
        }
    };
}

impl ResourceExt for Resource {
    fn rtype(&self) -> RType {
        impl_resource_dispatch!(self, rtype)
    }

    fn owner(&self) -> Option<ResourceLink> {
        impl_resource_dispatch!(self, owner)
    }

    fn delete_link(&mut self, rlink: &ResourceLink) {
        impl_resource_dispatch!(self, delete_link, rlink);
    }
}

pub trait ResourceExt {
    fn rtype(&self) -> RType;

    fn owner(&self) -> Option<ResourceLink> {
        None
    }

    fn delete_link(&mut self, _rlink: &ResourceLink) {}
}

impl ResourceExt for BehaviorInstance {
    fn rtype(&self) -> RType {
        RType::BehaviorInstance
    }
}

impl ResourceExt for BehaviorScript {
    fn rtype(&self) -> RType {
        RType::BehaviorScript
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

    fn delete_link(&mut self, rlink: &ResourceLink) {
        self.children.remove(rlink);
        self.services.remove(rlink);
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

    fn delete_link(&mut self, rlink: &ResourceLink) {
        self.services.remove(rlink);
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

    fn delete_link(&mut self, rlink: &ResourceLink) {
        self.locations
            .service_locations
            .retain(|sl| sl.service != *rlink);
        self.channels
            .retain(|chan| !chan.members.iter().any(|c| c.service == *rlink));
        self.light_services.retain(|ls| ls != rlink);
    }
}

impl ResourceExt for GeofenceClient {
    fn rtype(&self) -> RType {
        RType::GeofenceClient
    }
}

impl ResourceExt for Geolocation {
    fn rtype(&self) -> RType {
        RType::Geolocation
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
}

impl ResourceExt for PublicImage {
    fn rtype(&self) -> RType {
        RType::PublicImage
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

    fn delete_link(&mut self, rlink: &ResourceLink) {
        self.children.remove(rlink);
        self.services.remove(rlink);
    }
}

impl ResourceExt for Scene {
    fn rtype(&self) -> RType {
        RType::Scene
    }
}

impl ResourceExt for SmartScene {
    fn rtype(&self) -> RType {
        RType::SmartScene
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

    fn delete_link(&mut self, rlink: &ResourceLink) {
        self.children.remove(rlink);
        self.services.remove(rlink);
    }
}

impl ResourceExt for CameraMotion {
    fn rtype(&self) -> RType {
        RType::CameraMotion
    }
}

impl ResourceExt for Contact {
    fn rtype(&self) -> RType {
        RType::Contact
    }
}

impl ResourceExt for MatterFabric {
    fn rtype(&self) -> RType {
        RType::MatterFabric
    }
}

impl ResourceExt for ServiceGroup {
    fn rtype(&self) -> RType {
        RType::ServiceGroup
    }
}

impl ResourceExt for Tamper {
    fn rtype(&self) -> RType {
        RType::Tamper
    }
}

impl ResourceExt for ZgpConnectivity {
    fn rtype(&self) -> RType {
        RType::ZgpConnectivity
    }
}
