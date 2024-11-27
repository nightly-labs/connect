use super::{
    accountRemovalConfirmation::DELETE_ACCOUNT_EMAIL_TEMPLATE,
    emailConfirmation::EMAIL_CONFIRMATION_TEMPLATE, resetPassword::RESET_PASSWORD_TEMPLATE,
    teamInviteNotification::TEAM_INVITE_NOTIFICATION_TEMPLATE,
    teamLeavingNotification::TEAM_LEAVING_NOTIFICATION_TEMPLATE,
    teamRemovalNotification::TEAM_REMOVAL_NOTIFICATION_TEMPLATE,
};
use std::collections::HashMap;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Templates {
    EmailConfirmation,
    ResetPassword,
    TeamInviteNotification,
    TeamRemovalNotification,
    TeamLeavingNotification,
    AccountRemovalNotification,
}

pub fn get_templates() -> HashMap<Templates, String> {
    let mut templates = HashMap::new();

    templates.insert(
        Templates::EmailConfirmation,
        EMAIL_CONFIRMATION_TEMPLATE.to_string(),
    );
    templates.insert(
        Templates::ResetPassword,
        RESET_PASSWORD_TEMPLATE.to_string(),
    );
    templates.insert(
        Templates::TeamInviteNotification,
        TEAM_INVITE_NOTIFICATION_TEMPLATE.to_string(),
    );
    templates.insert(
        Templates::TeamRemovalNotification,
        TEAM_REMOVAL_NOTIFICATION_TEMPLATE.to_string(),
    );
    templates.insert(
        Templates::TeamLeavingNotification,
        TEAM_LEAVING_NOTIFICATION_TEMPLATE.to_string(),
    );
    templates.insert(
        Templates::AccountRemovalNotification,
        DELETE_ACCOUNT_EMAIL_TEMPLATE.to_string(),
    );

    templates
}
