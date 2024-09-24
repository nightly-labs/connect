use crate::mailer::{
    mail_requests::{SendEmailResponse, TeamRemovalNotification},
    request_handler::utils::create_message,
    templates::templates::Templates,
};
use lettre::{message::Mailbox, SmtpTransport, Transport};
use log::{error, warn};
use std::{collections::HashMap, sync::Arc};

pub fn send_team_removal_notification(
    templates: &Arc<HashMap<Templates, String>>,
    mailbox: Mailbox,
    mail_sender: &Arc<SmtpTransport>,
    request: &TeamRemovalNotification,
) -> SendEmailResponse {
    let html = match templates.get(&Templates::TeamRemovalNotification) {
        Some(template) => template
            .replace("EMAIL_TEAM_NAME", &request.team_name)
            .replace("EMAIL_ADMIN_ADDRESS", &request.remover_email),
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
        "Nightly Connect Cloud - Removed from team".to_string(),
    ) {
        Ok(message) => {
            if let Err(e) = mail_sender.send(&message) {
                warn!("MAILER: Failed to send team removal notification: {:?}", e);
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
                "MAILER: Failed to create team removal notification: {:?}",
                err
            );
            return SendEmailResponse {
                error_message: Some("Internal Error".to_string()),
            };
        }
    }
}
