use crate::mailer::{
    mail_requests::{SendEmailResponse, TeamInviteNotification},
    request_handler::utils::create_message,
    templates::templates::Templates,
};
use lettre::{message::Mailbox, SmtpTransport, Transport};
use log::{error, warn};
use std::{collections::HashMap, sync::Arc};

pub fn send_team_invite_notification(
    templates: &Arc<HashMap<Templates, String>>,
    mailbox: Mailbox,
    mail_sender: &Arc<SmtpTransport>,
    request: &TeamInviteNotification,
) -> SendEmailResponse {
    let html = match templates.get(&Templates::TeamInviteNotification) {
        // TODO For now simply pass team_name, fix when template is ready, this will require two fields to update
        // 1. message created from request in format "{inviter_email} invited you to join {team_name} on Nightly Connect Cloud"
        // 2. link which will navigate user to his invites page
        Some(template) => template
            .replace("EMAIL_TEAM_LINK", "https://cloud.nightly.app/settings")
            .replace("EMAIL_TEAM_NAME", &request.team_name),
        None => {
            // Only possible if someone messes with the templates, print error and go along
            error!(
                "MAILER: Could not find team invite notification template under: {:?}",
                Templates::TeamInviteNotification
            );
            return SendEmailResponse {
                error_message: Some("Internal Error".to_string()),
            };
        }
    };

    match create_message(
        html,
        mailbox,
        &request.email,
        "Nightly Connect Cloud - New team invite".to_string(),
    ) {
        Ok(message) => {
            if let Err(e) = mail_sender.send(&message) {
                warn!("MAILER: Failed to send team invite notification: {:?}", e);
                return SendEmailResponse {
                    error_message: Some("Internal Error".to_string()),
                };
            } else {
                return SendEmailResponse {
                    error_message: None,
                };
            }
        }
        Err(err) => {
            warn!(
                "MAILER: Failed to create team invite notification: {:?}",
                err
            );
            return SendEmailResponse {
                error_message: Some("Internal Error".to_string()),
            };
        }
    }
}
