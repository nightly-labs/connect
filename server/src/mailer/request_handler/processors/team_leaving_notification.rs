use crate::mailer::{
    mail_requests::{SendEmailResponse, TeamLeavingNotification},
    request_handler::utils::create_message,
    templates::templates::Templates,
};
use lettre::{message::Mailbox, SmtpTransport, Transport};
use log::{error, warn};
use std::{collections::HashMap, sync::Arc};

pub fn send_team_leaving_notification(
    templates: &Arc<HashMap<Templates, String>>,
    mailbox: Mailbox,
    mail_sender: &Arc<SmtpTransport>,
    request: &TeamLeavingNotification,
) -> SendEmailResponse {
    let html = match templates.get(&Templates::TeamLeavingNotification) {
        Some(template) => template.replace(
            "TEAM_LEAVING_MESSAGE_TO_REPLACE",
            format!("You left the team {}", request.team_name).as_str(),
        ),
        None => {
            // Only possible if someone messes with the templates, print error and go along
            error!(
                "MAILER: Could not find team leaving notification template under: {:?}",
                Templates::TeamLeavingNotification
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
        "Nightly Connect Cloud - Left the team".to_string(),
    ) {
        Ok(message) => {
            if let Err(e) = mail_sender.send(&message) {
                warn!("MAILER: Failed to send team leaving notification: {:?}", e);
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
                "MAILER: Failed to create team leaving notification: {:?}",
                err
            );
            return SendEmailResponse {
                error_message: Some("Internal Error".to_string()),
            };
        }
    }
}
