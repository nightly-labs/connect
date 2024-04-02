use super::mailer::Mailer;
use crate::env::{MAILER_ADDRESS, MAILER_PASSWORD};
use anyhow::Result;

pub async fn run() -> Result<Mailer> {
    let mailer = Mailer::init(MAILER_ADDRESS().to_string(), MAILER_PASSWORD().to_string()).await;

    Ok(mailer)
}
