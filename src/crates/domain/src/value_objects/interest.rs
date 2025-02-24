use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::hash::Hash;

use crate::value_objects::UserId;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Interest {
    pub user_id: UserId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub followed_at: DateTime<Utc>,
}

impl Interest {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            followed_at: Utc::now(),
        }
    }
}

impl Hash for Interest {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.user_id.hash(state);
    }
}

impl PartialEq for Interest {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl PartialEq<UserId> for Interest {
    fn eq(&self, other: &UserId) -> bool {
        &self.user_id == other
    }
}

impl Eq for Interest {}

impl Borrow<UserId> for Interest {
    fn borrow(&self) -> &UserId {
        &self.user_id
    }
}
