use database::tables::team_invites::table_struct::TeamInvite as DbTeamInvite;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct TeamInvite {
    pub creator_email: String,
    pub team_name: String,
    pub user_email: String,
    pub created_at: String,
}

impl From<DbTeamInvite> for TeamInvite {
    fn from(db_team_invite: DbTeamInvite) -> Self {
        Self {
            creator_email: db_team_invite.admin_email,
            team_name: db_team_invite.team_name,
            user_email: db_team_invite.user_email,
            created_at: db_team_invite.created_at.to_string(),
        }
    }
}
