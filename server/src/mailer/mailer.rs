use super::templates::templates::{get_templates, Templates};
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Address, SmtpTransport,
};
use std::{collections::HashMap, sync::Arc};
use strum::IntoEnumIterator;

pub struct Mailer {
    pub transport: Arc<SmtpTransport>,
    pub templates: Arc<HashMap<Templates, String>>,
    pub mailbox: Mailbox,
}

impl Mailer {
    pub async fn init(username: String, password: String) -> Self {
        let creds = Credentials::new(username.clone(), password);
        let mailer = SmtpTransport::relay("mail.privateemail.com")
            .expect("Could not create mailer")
            .credentials(creds)
            .build();

        // load templates
        let templates = get_templates();

        // just in case make sure all templates are loaded
        Templates::iter().for_each(|template| {
            assert!(templates.contains_key(&template));
        });

        let address = username.parse::<Address>().expect("Invalid address");
        let mailbox = Mailbox::new(Some("Nightly".to_string()), address);

        Self {
            transport: Arc::new(mailer),
            templates: Arc::new(templates),
            mailbox,
        }
    }
}
