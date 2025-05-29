/* pub mod option_empty_str { */
/*     use serde::{Deserialize, Deserializer, Serialize, Serializer}; */

/*     pub fn serialize<T, S>(v: &Option<T>, serializer: S) -> Result<S::Ok, S::Error> */
/*     where */
/*         T: Serialize, */
/*         S: Serializer, */
/*     { */
/*         match v { */
/*             None => "".serialize(serializer), */
/*             Some(d) => d.serialize(serializer), */
/*         } */
/*     } */

/*     pub fn deserialize<'de, T, D>(d: D) -> Result<Option<T>, D::Error> */
/*     where */
/*         T: Deserialize<'de> + TryFrom<String>, */
/*         T::Error: ToString, */
/*         D: Deserializer<'de>, */
/*     { */
/*         let val = String::deserialize(d)?; */
/*         if val.is_empty() { */
/*             Ok(None) */
/*         } else { */
/*             Ok(Some(T::try_from(val).map_err(|err| { */
/*                 serde::de::Error::custom(err.to_string()) */
/*             })?)) */
/*         } */
/*     } */
/* } */

pub mod option_ipaddr_or_empty {
    use std::net::IpAddr;

    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[allow(clippy::ref_option)]
    pub fn serialize<S>(v: &Option<IpAddr>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match v {
            None => "".serialize(serializer),
            Some(d) => d.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<IpAddr>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = String::deserialize(d)?;
        if val.is_empty() {
            Ok(None)
        } else {
            Ok(Some(val.parse().map_err(Error::custom)?))
        }
    }
}

pub mod bool_as_int {
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(v: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(u8::from(*v))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u8::deserialize(deserializer)?;
        Ok(val > 0)
    }
}
