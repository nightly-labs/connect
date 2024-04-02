use super::{
    emailConfirmation::EMAIL_CONFIRMATION_TEMPLATE, resetPassword::RESET_PASSWORD_TEMPLATE,
};
use std::collections::HashMap;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Templates {
    EmailConfirmation,
    ResetPassword,
}

pub fn get_templates() -> HashMap<Templates, String> {
    let mut templates = HashMap::new();

    templates.insert(
        Templates::EmailConfirmation,
        EMAIL_CONFIRMATION_TEMPLATE.to_string(),
    );
    templates.insert(
        Templates::ResetPassword,
        RESET_PASSWORD_TEMPLATE.to_string(),
    );

    templates
}
