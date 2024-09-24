use crate::mailer::{
    mail_requests::{EmailConfirmationRequest, SendEmailResponse},
    request_handler::utils::create_message,
    templates::templates::Templates,
};
use lettre::{message::Mailbox, SmtpTransport, Transport};
use log::{error, warn};
use std::sync::Arc;

pub fn send_email_confirmation(
    templates: &Arc<std::collections::HashMap<Templates, String>>,
    mailbox: Mailbox,
    mail_sender: &Arc<SmtpTransport>,
    request: &EmailConfirmationRequest,
    date: String,
    time: String,
) -> SendEmailResponse {
    let html = match templates.get(&Templates::EmailConfirmation) {
        Some(template) => template
            .replace("EMAIL_CONFIRMATION_CODE", &request.code)
            .replace("EMAIL_ACTION_DEVICE", &request.device)
            .replace("EMAIL_ACTION_BROWSER", &request.browser)
            .replace("EMAIL_ACTION_DATE", &date)
            .replace("EMAIL_ACTION_TIME", &time),
        None => {
            // Only possible if someone messes with the templates, print error and go along
            error!(
                "MAILER: Could not find email confirmation template under: {:?}",
                Templates::EmailConfirmation
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
        "Nightly Connect Cloud - Confirm your email address".to_string(),
    ) {
        Ok(message) => {
            if let Err(e) = mail_sender.send(&message) {
                warn!("MAILER: Failed to send email confirmation: {:?}", e);
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
            warn!("MAILER: Failed to create email: {:?}", err);
            return SendEmailResponse {
                error_message: Some("Internal Error".to_string()),
            };
        }
    }
}
