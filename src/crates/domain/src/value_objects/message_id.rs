use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Display, Debug, Clone, Hash, PartialEq, Eq)]
#[display("{_0}")]
pub struct MessageId(pub String);

impl Deref for MessageId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
