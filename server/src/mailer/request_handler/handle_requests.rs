use super::processors::{
    email_confirmation::send_email_confirmation, reset_password::send_password_reset,
};
use crate::mailer::{
    mail_requests::{SendEmailRequest, SendEmailResponse},
    mailer::Mailer,
};

impl Mailer {
    pub fn handle_email_request(&self, request: SendEmailRequest) -> SendEmailResponse {
        let templates = self.templates.clone();
        let mail_sender = self.transport.clone();
        let mailbox = self.mailbox.clone();

        match request {
            SendEmailRequest::EmailConfirmation(request) => {
                return send_email_confirmation(
                    &templates,
                    mailbox.clone(),
                    &mail_sender,
                    &request,
                );
            }
            SendEmailRequest::ResetPassword(request) => {
                return send_password_reset(&templates, mailbox.clone(), &mail_sender, &request);
            }
        }
    }
}
