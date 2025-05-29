use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OptFlags: u16 {
        const DEBUG_HOST       = 0x0100;
        const DEBUG            = 0x0080;
        const ALEXA            = 0x0040;
        /* const _             = 0x0020; // indicated now removed Blynk support, may be reused to indicate another build-time option */
        const USERMOD_CRONIXIE = 0x0010;
        const FILESYSTEM       = 0x0008;
        const HUESYNC          = 0x0004;
        const ADALIGHT         = 0x0002;
        const OTA              = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SegCap: u16 {
        /// Light segment supports RGB colors
        const RGB = 0x01;
        /// Light segment supports White light
        const W   = 0x02;
        /// Light segment supports Correlated Color Temperature
        const CCT = 0x04;
    }
}

impl Serialize for OptFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for OptFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = u16::deserialize(deserializer)?;
        Self::from_bits(val).ok_or_else(|| serde::de::Error::custom("Invalid bitflags"))
    }
}

impl Serialize for SegCap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for SegCap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = u16::deserialize(deserializer)?;
        Self::from_bits(val).ok_or_else(|| serde::de::Error::custom("Invalid bitflags"))
    }
}
