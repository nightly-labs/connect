use crate::mailer::{
    mail_requests::{ResetPasswordRequest, SendEmailResponse},
    request_handler::utils::create_message,
    templates::templates::Templates,
};
use lettre::{message::Mailbox, SmtpTransport, Transport};
use log::{error, warn};
use std::sync::Arc;

pub fn send_password_reset(
    templates: &Arc<std::collections::HashMap<Templates, String>>,
    mailbox: Mailbox,
    mail_sender: &Arc<SmtpTransport>,
    request: &ResetPasswordRequest,
    date: String,
    time: String,
) -> SendEmailResponse {
    let html = match templates.get(&Templates::ResetPassword) {
        Some(template) => template
            .replace("EMAIL_PASSWORD_RESET_CODE", &request.code)
            .replace("EMAIL_ACTION_DEVICE", &request.device)
            .replace("EMAIL_ACTION_BROWSER", &request.browser)
            .replace("EMAIL_ACTION_DATE", &date)
            .replace("EMAIL_ACTION_TIME", &time),
        None => {
            // Only possible if someone messes with the templates, print error and go along
            error!(
                "MAILER: Could not find email confirmation template under: {:?}",
                Templates::ResetPassword
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
        "Nightly Connect Cloud - Change password request".to_string(),
    ) {
        Ok(message) => {
            if let Err(e) = mail_sender.send(&message) {
                warn!("MAILER: Failed to send password reset email: {:?}", e);
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
