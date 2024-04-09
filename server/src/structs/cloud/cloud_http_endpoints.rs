use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum HttpCloudEndpoint {
    #[serde(rename = "/register_new_app")]
    RegisterNewApp,
    #[serde(rename = "/register_with_password")]
    RegisterWithPasswordStart,
    #[serde(rename = "/login_with_password_start")]
    RegisterWithPasswordFinish,
    #[serde(rename = "/login_with_password_finish")]
    LoginWithPassword,
    #[serde(rename = "/login_with_google")]
    LoginWithGoogle,
    #[serde(rename = "/register_new_team")]
    RegisterNewTeam,
    #[serde(rename = "/remove_user_from_team")]
    RemoveUserFromTeam,
    #[serde(rename = "/get_user_joined_teams")]
    GetUserJoinedTeams,
    #[serde(rename = "/events")]
    Events,
    #[serde(rename = "/invite_user_to_team")]
    InviteUserToTeam,
    #[serde(rename = "/accept_team_invite")]
    AcceptTeamInvite,
    #[serde(rename = "/get_team_user_invites")]
    GetTeamUserInvites,
    #[serde(rename = "/get_user_team_invites")]
    GetUserTeamInvites,
    #[serde(rename = "/cancel_team_user_invite")]
    CancelTeamUserInvite,
    #[serde(rename = "/cancel_user_team_invite")]
    CancelUserTeamInvite,
    #[serde(rename = "/get_app_events")]
    GetEvents,
    #[serde(rename = "/reset_password_start")]
    ResetPasswordStart,
    #[serde(rename = "/reset_password_finish")]
    ResetPasswordFinish,
    #[serde(rename = "/verify_domain_start")]
    VerifyDomainStart,
    #[serde(rename = "/verify_domain_finish")]
    VerifyDomainFinish,
    #[serde(rename = "/remove_whitelisted_domain")]
    RemoveWhitelistedDomain,
    #[serde(rename = "/register_with_passkey_start")]
    RegisterWithPasskeyStart,
    #[serde(rename = "/register_with_passkey_finish")]
    RegisterWithPasskeyFinish,
    #[serde(rename = "/reset_passkey_start")]
    ResetPasskeyStart,
    #[serde(rename = "/reset_passkey_finish")]
    ResetPasskeyFinish,
    #[serde(rename = "/get_passkey_challenge")]
    GetPasskeyChallenge,
    #[serde(rename = "/delete_passkey")]
    DeletePasskey,
    #[serde(rename = "/add_passkey_start")]
    AddPasskeyStart,
    #[serde(rename = "/add_passkey_finish")]
    AddPasskeyFinish,
    #[serde(rename = "/get_user_metadata")]
    GetUserMetadata,
    #[serde(rename = "/get_team_metadata")]
    GetTeamMetadata,
    #[serde(rename = "/get_team_user_privileges")]
    GetTeamUserPrivileges,
    #[serde(rename = "/change_user_privileges")]
    ChangeUserPrivileges,
}

impl HttpCloudEndpoint {
    pub fn to_string(&self) -> String {
        match self {
            HttpCloudEndpoint::RegisterNewApp => "/register_new_app".to_string(),
            HttpCloudEndpoint::RegisterWithPasswordStart => {
                "/register_with_password_start".to_string()
            }
            HttpCloudEndpoint::RegisterWithPasswordFinish => {
                "/login_with_password_finish".to_string()
            }
            HttpCloudEndpoint::LoginWithPassword => "/login_with_password".to_string(),
            HttpCloudEndpoint::LoginWithGoogle => "/login_with_google".to_string(),
            HttpCloudEndpoint::RegisterNewTeam => "/register_new_team".to_string(),
            HttpCloudEndpoint::RemoveUserFromTeam => "/remove_user_from_team".to_string(),
            HttpCloudEndpoint::GetUserJoinedTeams => "/get_user_joined_teams".to_string(),
            HttpCloudEndpoint::Events => "/events".to_string(),
            HttpCloudEndpoint::InviteUserToTeam => "/invite_user_to_team".to_string(),
            HttpCloudEndpoint::AcceptTeamInvite => "/accept_team_invite".to_string(),
            HttpCloudEndpoint::GetTeamUserInvites => "/get_team_user_invites".to_string(),
            HttpCloudEndpoint::GetUserTeamInvites => "/get_user_team_invites".to_string(),
            HttpCloudEndpoint::CancelTeamUserInvite => "/cancel_team_user_invite".to_string(),
            HttpCloudEndpoint::CancelUserTeamInvite => "/cancel_user_team_invite".to_string(),
            HttpCloudEndpoint::GetEvents => "/get_app_events".to_string(),
            HttpCloudEndpoint::ResetPasswordStart => "/reset_password_start".to_string(),
            HttpCloudEndpoint::ResetPasswordFinish => "/reset_password_finish".to_string(),
            HttpCloudEndpoint::VerifyDomainStart => "/verify_domain_start".to_string(),
            HttpCloudEndpoint::VerifyDomainFinish => "/verify_domain_finish".to_string(),
            HttpCloudEndpoint::RemoveWhitelistedDomain => "/remove_whitelisted_domain".to_string(),
            HttpCloudEndpoint::RegisterWithPasskeyStart => {
                "/register_with_passkey_start".to_string()
            }
            HttpCloudEndpoint::RegisterWithPasskeyFinish => {
                "/register_with_passkey_finish".to_string()
            }
            HttpCloudEndpoint::ResetPasskeyStart => "/reset_passkey_start".to_string(),
            HttpCloudEndpoint::ResetPasskeyFinish => "/reset_passkey_finish".to_string(),
            HttpCloudEndpoint::DeletePasskey => "/delete_passkey".to_string(),
            HttpCloudEndpoint::GetPasskeyChallenge => "/get_passkey_challenge".to_string(),
            HttpCloudEndpoint::AddPasskeyStart => "/add_passkey_start".to_string(),
            HttpCloudEndpoint::AddPasskeyFinish => "/add_passkey_finish".to_string(),
            HttpCloudEndpoint::GetUserMetadata => "/get_user_metadata".to_string(),
            HttpCloudEndpoint::GetTeamMetadata => "/get_team_metadata".to_string(),
            HttpCloudEndpoint::GetTeamUserPrivileges => "/get_team_user_privileges".to_string(),
            HttpCloudEndpoint::ChangeUserPrivileges => "/change_user_privileges".to_string(),
        }
    }
}
