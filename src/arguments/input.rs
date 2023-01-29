use clap::Parser;
use dialoguer::theme::ColorfulTheme;



use super::common_trait::{Run,Default};

/// Prompt that takes user input and returns a string.
#[derive(Debug, Parser)]
pub struct Input {
    /// Message for the prompt
    #[clap(short, long)]
    message: String,

    /// Default value for the prompt
    #[clap(short, long)]
    default: Option<String>,

    /// Allow empty input. Conflicts with `default`
    #[clap(short, long, conflicts_with = "default")]
    allow_empty: bool,

}

impl Run<String> for Input {
     fn run(&self) -> Result<String, Box<dyn std::error::Error>>  {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);

        input
            .with_prompt(&self.message)
            .allow_empty(self.allow_empty);

        if self.default.is_some() {
            input.default(self.default.as_ref().unwrap().to_string());
        }

        let value = input.interact_text()?;

        println!("{}", &value);

        Ok(value)
    }
}

impl Default for Input {
    fn default(message:&str, can_be_nullable:Option<bool>) -> Self {
        Self {
            message: message.to_string(),
            default: None,
        
            allow_empty: can_be_nullable.unwrap_or(false),
        }
    }
}
