
pub struct HashVisitor;

use core::fmt;
use core::convert::TryInto;

use serde::de::{self, Visitor};

use crate::computation::{SHashType,HashType};

impl<'de> Visitor<'de> for HashVisitor {
    type Value = HashType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a bytes array")
    }

    /*
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use core::u64;
        if value >= u64::from(u64::MIN) && value <= u64::from(u64::MAX) {
            Ok(value as u64)
        } else {
            Err(E::custom(alloc::format!("u64 out of range: {}", value)))
        }
    }
    */

    fn visit_bytes<E> (self, value: &[u8]) -> Result<Self::Value, E> 
        where E: de::Error
    {
    
        Ok (SHashType { data : value.try_into().expect("incorrect bytes arrays")} )
    
    }

}