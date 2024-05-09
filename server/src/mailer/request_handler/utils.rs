use anyhow::Result;
use lettre::{
  message::{header::ContentType, Mailbox},
  Message,
};

pub fn create_message(
  body: String,
  mailbox: Mailbox,
  receiver: &String,
  subject: String,
) -> Result<Message> {
  Message::builder()
    .from(mailbox)
    .to(
      format!("Hello <{receiver}>")
        .parse()
        .map_err(|e| anyhow::anyhow!("Could not parse to: {:?}", e))?,
    )
    .subject(subject)
    .header(ContentType::TEXT_HTML)
    .body(body)
    .map_err(|e| anyhow::anyhow!("Could not build email: {:?}", e))
}
