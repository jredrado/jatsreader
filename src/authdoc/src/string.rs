use minicbor::{Encode,Decode};
use serde::{Deserialize, Serialize};
use nanoserde::ToJSON;

use std::cmp::Ordering;
use std::ops::Deref;
use std::fmt;

#[derive(Serialize, Deserialize, Encode, Decode, ToJSON, Debug, Eq, Clone, Hash, PartialOrd)]
pub struct NodeString {
    #[n(0)] pub inner: String
}


impl Deref for NodeString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Display for NodeString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<&str> for NodeString {
    fn from(item: &str) -> Self {
        NodeString  { inner: String::from(item) }
    }
}

impl PartialEq for NodeString {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Ord for NodeString {
    fn cmp(&self, other:&Self) -> Ordering {
        return self.inner.cmp(&other.inner)
    }
}