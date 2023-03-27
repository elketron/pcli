use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub location: PathBuf,
}
