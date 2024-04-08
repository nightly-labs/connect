use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::joined_team::TeamId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct TeamMetadata {
    pub creator_email: String,
    pub team_id: TeamId,
    pub team_name: String,
    pub personal_team: bool,
    pub created_at: DateTime<Utc>,
}
