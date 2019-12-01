use std::fmt::{Formatter, Result as FmtResult};

use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::de::{Deserialize, Deserializer, Error, Unexpected, Visitor};
use serde_json::{to_value, Result as JsonResult, Value};

/// Read a [Message](enum.Message.html) from a slice.
///
/// Invalid JSON or JSONRPC messages are reported as [Broken](enum.Broken.html).
pub fn from_slice(s: &[u8]) -> Parsed {
    decoded_to_parsed(::serde_json::de::from_slice(s))
}

/// Read a [Message](enum.Message.html) from a string.
///
/// Invalid JSON or JSONRPC messages are reported as [Broken](enum.Broken.html).
pub fn from_str(s: &str) -> Parsed {
    println!("Received {:?}", s);
    from_slice(s.as_bytes())
}

impl Into<String> for Message {
    fn into(self) -> String {
        ::serde_json::ser::to_string(&self).unwrap()
    }
}

impl Into<Vec<u8>> for Message {
    fn into(self) -> Vec<u8> {
        ::serde_json::ser::to_vec(&self).unwrap()
    }
}
