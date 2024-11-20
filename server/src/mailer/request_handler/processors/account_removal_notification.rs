use crate::mailer::{
    mail_requests::{DeleteAccountNotification, SendEmailResponse},
    request_handler::utils::create_message,
    templates::templates::Templates,
};
use lettre::{message::Mailbox, SmtpTransport, Transport};
use log::{error, warn};
use std::{collections::HashMap, sync::Arc};

pub fn send_account_removal_notification(
    templates: &Arc<HashMap<Templates, String>>,
    mailbox: Mailbox,
    mail_sender: &Arc<SmtpTransport>,
    request: &DeleteAccountNotification,
    date: String,
    time: String,
) -> SendEmailResponse {
    let html = match templates.get(&Templates::AccountRemovalNotification) {
        Some(template) => template
            .replace("EMAIL_ACTION_DEVICE", &request.device)
            .replace("EMAIL_ACTION_BROWSER", &request.browser)
            .replace("EMAIL_ACTION_DATE", &date)
            .replace("EMAIL_ACTION_TIME", &time),
        None => {
            // Only possible if someone messes with the templates, print error and go along
            error!(
                "MAILER: Could not find account removal notification template under: {:?}",
                Templates::AccountRemovalNotification
            );
            return SendEmailResponse {
                error_message: Some("Internal Error".to_string()),
            };
        }
    };
    println!("mailbox: {:?}", mailbox);
    match create_message(
        html,
        mailbox,
        &request.email,
        "Nightly Connect Cloud - Remove your account".to_string(),
    ) {
        Ok(message) => {
            if let Err(e) = mail_sender.send(&message) {
                warn!("MAILER: Failed to send account removal notification: {:?}", e);
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
                "MAILER: Failed to create account removal notification: {:?}",
                err
            );
            return SendEmailResponse {
                error_message: Some("Internal Error".to_string()),
            };
        }
    }
}
