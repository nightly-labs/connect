use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SendEmailRequest {
    EmailConfirmation(EmailConfirmationRequest),
    ResetPassword(ResetPasswordRequest),
    TeamInvite(TeamInviteNotification),
    TeamRemoval(TeamRemovalNotification),
    LeaveTeam(TeamLeavingNotification),
    DeleteAccount(DeleteAccountNotification),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SendEmailResponse {
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailConfirmationRequest {
    pub email: String,
    pub code: String,
    pub device: String,
    pub browser: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub code: String,
    pub device: String,
    pub browser: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamInviteNotification {
    pub email: String,
    pub team_name: String,
    pub inviter_email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamRemovalNotification {
    pub email: String,
    pub team_name: String,
    pub remover_email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamLeavingNotification {
    pub email: String,
    pub team_name: String,
    pub device: String,
    pub browser: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeleteAccountNotification {
    pub email: String,
    pub code: String,
    pub device: String,
    pub browser: String,
}
