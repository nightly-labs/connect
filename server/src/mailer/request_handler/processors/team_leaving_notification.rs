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
    date: String,
    time: String,
) -> SendEmailResponse {
    let html = match templates.get(&Templates::TeamLeavingNotification) {
        Some(template) => template
            .replace("EMAIL_TEAM_NAME", &request.team_name)
            .replace("EMAIL_ACTION_DEVICE", &request.device)
            .replace("EMAIL_ACTION_BROWSER", &request.browser)
            .replace("EMAIL_ACTION_DATE", &date)
            .replace("EMAIL_ACTION_TIME", &time),
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
