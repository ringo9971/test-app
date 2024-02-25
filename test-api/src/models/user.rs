use crate::models::model::HasUid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_uid: Option<String>,
    pub name: String,
}

impl HasUid for User {
    fn add_uid(&mut self, uid: &str) {
        self.user_uid = Some(uid.to_string());
    }
}
