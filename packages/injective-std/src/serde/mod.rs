pub mod as_str {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str(&s).map_err(de::Error::custom)
    }

    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        serializer.serialize_str(&value.to_string())
    }
}

pub mod enum_i32 {
    use serde::de::{self, IntoDeserializer, Visitor};
    use serde::{Deserialize, Deserializer};
    use std::{fmt, marker::PhantomData};

    pub fn deserialize<'de, E, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
        E: Deserialize<'de> + Into<i32>,
    {
        struct EnumVisitor<E>(PhantomData<E>);

        impl<'de, E> Visitor<'de> for EnumVisitor<E>
        where
            E: Deserialize<'de> + Into<i32>,
        {
            type Value = i32;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an enum name or integer")
            }

            fn visit_i64<Err>(self, value: i64) -> Result<i32, Err>
            where
                Err: de::Error,
            {
                if value < i32::MIN as i64 || value > i32::MAX as i64 {
                    return Err(Err::custom("enum value out of range for i32"));
                }
                Ok(value as i32)
            }

            fn visit_u64<Err>(self, value: u64) -> Result<i32, Err>
            where
                Err: de::Error,
            {
                if value > i32::MAX as u64 {
                    return Err(Err::custom("enum value out of range for i32"));
                }
                Ok(value as i32)
            }

            fn visit_str<Err>(self, value: &str) -> Result<i32, Err>
            where
                Err: de::Error,
            {
                if let Ok(value) = value.parse::<i32>() {
                    return Ok(value);
                }
                let enum_val: E = Deserialize::deserialize(value.into_deserializer())?;
                Ok(enum_val.into())
            }

            fn visit_string<Err>(self, value: String) -> Result<i32, Err>
            where
                Err: de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_any(EnumVisitor::<E>(PhantomData))
    }
}
