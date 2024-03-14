use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type TeamId = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct JoinedTeam {
    pub team_id: TeamId,
    pub team_name: String,
    pub creator_email: String,
    pub created_at: DateTime<Utc>,
    pub joined_at: DateTime<Utc>,
    pub personal: bool,
}
