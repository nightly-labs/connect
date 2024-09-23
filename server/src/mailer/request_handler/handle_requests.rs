use log::error;

use super::processors::{
    email_confirmation::send_email_confirmation, reset_password::send_password_reset,
    team_invite_notification::send_team_invite_notification,
    team_leaving_notification::send_team_leaving_notification,
    team_removal_notification::send_team_removal_notification,
};
use crate::{
    env::MAILER_ACTIVE,
    mailer::{mail_requests::SendEmailRequest, mailer::Mailer},
};

impl Mailer {
    pub fn handle_email_request(&self, request: &SendEmailRequest) {
        // Check if mailer is active, safe usage of flag, validation performed during state initialization
        if !MAILER_ACTIVE() {
            // Simulate success
            return;
        }

        let templates = self.templates.clone();
        let mail_sender = self.transport.clone();
        let mailbox = self.mailbox.clone();
        let request = request.clone();

        tokio::spawn(async move {
            match request {
                SendEmailRequest::EmailConfirmation(request) => {
                    if let Some(err) =
                        send_email_confirmation(&templates, mailbox.clone(), &mail_sender, &request)
                            .error_message
                    {
                        error!("Failed to send email: {:?}, request: {:?}", err, request);
                    }
                }
                SendEmailRequest::ResetPassword(request) => {
                    if let Some(err) =
                        send_password_reset(&templates, mailbox.clone(), &mail_sender, &request)
                            .error_message
                    {
                        error!("Failed to send email: {:?}, request: {:?}", err, request);
                    }
                }
                SendEmailRequest::TeamInvite(request) => {
                    if let Some(err) = send_team_invite_notification(
                        &templates,
                        mailbox.clone(),
                        &mail_sender,
                        &request,
                    )
                    .error_message
                    {
                        error!("Failed to send email: {:?}, request: {:?}", err, request);
                    }
                }
                SendEmailRequest::TeamRemoval(request) => {
                    if let Some(err) = send_team_removal_notification(
                        &templates,
                        mailbox.clone(),
                        &mail_sender,
                        &request,
                    )
                    .error_message
                    {
                        error!("Failed to send email: {:?}, request: {:?}", err, request);
                    }
                }
                SendEmailRequest::LeaveTeam(request) => {
                    if let Some(err) = send_team_leaving_notification(
                        &templates,
                        mailbox.clone(),
                        &mail_sender,
                        &request,
                    )
                    .error_message
                    {
                        error!("Failed to send email: {:?}, request: {:?}", err, request);
                    }
                }
            }
        });
        return;
    }
}
