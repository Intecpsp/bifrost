use packed_struct::prelude::*;

pub const DDP_PORT: u16 = 4048;

#[derive(PrimitiveEnum_u8, Debug, Clone, Copy)]
pub enum Version {
    V1 = 0b01,
}

#[derive(PrimitiveEnum_u8, Debug, Clone, Copy)]
pub enum DataType {
    Undefined = 0b000,
    RGB = 0b001,
    HSL = 0b010,
    RGBW = 0b011,
    Grayscale = 0b100,
}

#[derive(PrimitiveEnum_u8, Debug, Clone, Copy)]
pub enum BitsPerChannel {
    Undefined = 0,
    Bits1 = 1,
    Bits4 = 2,
    Bits8 = 3,
    Bits16 = 4,
    Bits24 = 5,
    Bits32 = 6,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Id {
    Reserved,
    Default,
    Custom(u8),
    JsonControl,
    JsonConfig,
    JsonStatus,
    DmxTransit,
    AllDevices,
}

impl PrimitiveEnum for Id {
    type Primitive = u8;

    #[allow(clippy::match_same_arms)]
    fn from_primitive(val: Self::Primitive) -> Option<Self> {
        match val {
            0 => Some(Self::Reserved),
            1 => Some(Self::Default),
            2..=245 => Some(Self::Custom(val)),
            246 => Some(Self::JsonControl),
            247..=249 => None,
            250 => Some(Self::JsonConfig),
            251 => Some(Self::JsonStatus),
            252..=253 => None,
            254 => Some(Self::DmxTransit),
            255 => Some(Self::AllDevices),
        }
    }

    fn to_primitive(&self) -> Self::Primitive {
        match self {
            Self::Reserved => 0,
            Self::Default => 1,
            Self::Custom(val) => *val,
            Self::JsonControl => 246,
            Self::JsonConfig => 250,
            Self::JsonStatus => 251,
            Self::DmxTransit => 254,
            Self::AllDevices => 255,
        }
    }

    fn from_str(_s: &str) -> Option<Self> {
        None
    }

    fn from_str_lower(_s: &str) -> Option<Self> {
        None
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(PackedStruct, Debug)]
#[packed_struct(size_bytes = "1", bit_numbering = "msb0")]
pub struct Flags {
    #[packed_field(bits = "0..=1", ty = "enum")]
    pub version: Version,

    /// timecode field added to end of header
    ///
    /// if T & P are set, Push at specified time.
    #[packed_field(bits = "3")]
    pub timecode: bool,

    /// Storage.
    ///
    /// If set, data comes from Storage, not data-field..
    #[packed_field(bits = "4")]
    pub storage: bool,

    /// Reply flag, marks reply to Query packet.
    ///
    /// Always set when any packet is sent by a Display.
    ///
    /// If Reply, Q flag is ignored.
    #[packed_field(bits = "5")]
    pub reply: bool,

    /// Query flag, requests len data from ID at offset (no data sent)
    ///
    /// If clear, is a Write buffer packet.
    #[packed_field(bits = "6")]
    pub query: bool,

    /// Push flag, for display synchronization, or marks last packet of Reply.
    #[packed_field(bits = "7")]
    pub push: bool,
}

impl Flags {
    pub const PUSH: Self = Self {
        version: Version::V1,
        timecode: false,
        storage: false,
        reply: false,
        query: false,
        push: true,
    };
}

#[derive(PackedStruct, Debug)]
#[packed_struct(size_bytes = "10", endian = "msb", bit_numbering = "msb0")]
pub struct DDPHeader {
    #[packed_field(bytes = "0")]
    pub flags: Flags,

    /// Sequence number from 1-15, or zero if not used.
    ///
    /// The sequence number should be incremented with each new packet sent.
    ///
    /// A sender can send duplicate packets with the same sequence number and DDP header for redundancy.
    ///
    /// A receiver can ignore duplicates received back-to-back.
    ///
    /// The sequence number is ignored if zero.
    #[packed_field(bits = "12..=15")]
    pub sequence: u8,

    /// C is 0 for standard types or 1 for Customer defined
    #[packed_field(bits = "16")]
    pub c: bool,

    #[packed_field(bits = "18..=20", ty = "enum")]
    pub datatype: DataType,

    #[packed_field(bits = "21..=23", ty = "enum")]
    pub bits: BitsPerChannel,

    /// 0 = reserved
    /// 1 = default output device
    /// 2-249 custom IDs, (possibly defined via JSON config)
    /// 246 = JSON control (read/write)
    /// 250 = JSON config  (read/write)
    /// 251 = JSON status  (read only)
    /// 254 = DMX transit
    /// 255 = all devices
    #[packed_field(bytes = "3", ty = "enum")]
    pub id: Id,

    /// Data offset in bytes
    pub offset: u32,

    /// Data length in bytes (size of data field when writing)
    ///
    /// For queries, this specifies size of data to read, no data field follows header.
    pub length: u16,
}

impl DDPHeader {
    pub const SIZE: usize = size_of::<<Self as PackedStruct>::ByteArray>();

    #[must_use]
    pub const fn new(flags: Flags, datatype: DataType, bits: BitsPerChannel) -> Self {
        Self {
            flags,
            sequence: 1,
            c: false,
            datatype,
            bits,
            id: Id::Default,
            offset: 0,
            length: 0,
        }
    }

    #[must_use]
    pub const fn with_flags(self, flags: Flags) -> Self {
        Self { flags, ..self }
    }

    #[must_use]
    pub const fn with_sequence(self, sequence: u8) -> Self {
        Self { sequence, ..self }
    }

    #[must_use]
    pub const fn with_id(self, id: Id) -> Self {
        Self { id, ..self }
    }

    #[must_use]
    pub const fn with_offset(self, offset: u32) -> Self {
        Self { offset, ..self }
    }

    #[must_use]
    pub const fn with_length(self, length: u16) -> Self {
        Self { length, ..self }
    }

    pub fn pack(&self) -> Result<[u8; Self::SIZE], PackingError> {
        let mut res = [0u8; Self::SIZE];
        self.pack_to_slice(&mut res)?;
        Ok(res)
    }
}
