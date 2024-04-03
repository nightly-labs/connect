use super::processors::{
    email_confirmation::send_email_confirmation, reset_password::send_password_reset,
    team_invite_notification::send_team_invite_notification,
    team_removal_notification::send_team_removal_notification,
};
use crate::mailer::{
    mail_requests::{SendEmailRequest, SendEmailResponse},
    mailer::Mailer,
};

impl Mailer {
    pub fn handle_email_request(&self, request: &SendEmailRequest) -> SendEmailResponse {
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
            SendEmailRequest::TeamInvite(request) => {
                return send_team_invite_notification(
                    &templates,
                    mailbox.clone(),
                    &mail_sender,
                    &request,
                );
            }
            SendEmailRequest::TeamRemoval(request) => {
                return send_team_removal_notification(
                    &templates,
                    mailbox.clone(),
                    &mail_sender,
                    &request,
                );
            }
        }
    }
}
