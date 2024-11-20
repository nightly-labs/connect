use chrono::Datelike;
use database::tables::utils::get_current_datetime;
use log::error;

use super::processors::{
    account_removal_notification::send_account_removal_notification, email_confirmation::send_email_confirmation, reset_password::send_password_reset, team_invite_notification::send_team_invite_notification, team_leaving_notification::send_team_leaving_notification, team_removal_notification::send_team_removal_notification
};
use crate::{
    env::MAILER_ACTIVE,
    mailer::{mail_requests::SendEmailRequest, mailer::Mailer},
};

fn get_date() -> (String, String) {
    let now = get_current_datetime();
    let day = now.day();
    let month = now.format("%B").to_string();
    let year = now.year();
    let time = now.format("%H:%M:%S").to_string();
    let day_suffix = match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    };
    let date_string = format!("{}{} {} {}", day, day_suffix, month, year);

    let time_string = time;
    return (date_string, time_string);
}

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
                    let date = get_date();
                    if let Some(err) = send_email_confirmation(
                        &templates,
                        mailbox.clone(),
                        &mail_sender,
                        &request,
                        date.0,
                        date.1,
                    )
                    .error_message
                    {
                        error!("Failed to send email: {:?}, request: {:?}", err, request);
                    }
                }
                SendEmailRequest::ResetPassword(request) => {
                    let date = get_date();
                    if let Some(err) = send_password_reset(
                        &templates,
                        mailbox.clone(),
                        &mail_sender,
                        &request,
                        date.0,
                        date.1,
                    )
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
                    let date = get_date();
                    if let Some(err) = send_team_leaving_notification(
                        &templates,
                        mailbox.clone(),
                        &mail_sender,
                        &request,
                        date.0,
                        date.1,
                    )
                    .error_message
                    {
                        error!("Failed to send email: {:?}, request: {:?}", err, request);
                    }
                }
                SendEmailRequest::DeleteAccount(request) => {
                    let date = get_date();
                    if let Some(err) = send_account_removal_notification(
                        &templates,
                        mailbox.clone(),
                        &mail_sender,
                        &request,
                        date.0,
                        date.1,
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
