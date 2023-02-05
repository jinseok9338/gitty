use clap::Parser;
use dialoguer::theme::ColorfulTheme;

use super::common_trait::{Default, Run};
/// Prompt that returns `true` or `false` (as strings)
#[derive(Debug, Parser)]
pub struct Confirm {
    /// Message for the prompt
    #[clap(short, long)]
    message: String,

    /// Makes the prompt cancellable with 'Esc' or 'q'
    #[clap(short, long)]
    cancel: bool,

    /// Sets the default value for the prompt as `true`
    #[clap(short, long)]
    default: bool,

    value: Option<bool>,
}

impl Run<bool, Box<dyn std::error::Error>> for Confirm {
    fn run(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Confirm::with_theme(&theme);

        input.with_prompt(&self.message).default(self.default);

        let ret = if self.cancel {
            input.interact_opt()?
        } else {
            Some(input.interact()?)
        };

        let value = ret.map_or_else(|| std::process::exit(1), |value| value);

        if value {
            println!("true");
            Ok(true)
        } else {
            println!("false");
            Ok(false)
        }
    }
}

impl Default<String> for Confirm {
    fn default(message: &str, _can_be_nullable: Option<bool>, _items: Option<Vec<String>>) -> Self {
        Self {
            message: message.to_string(),
            cancel: false,
            default: false,
            value: None,
        }
    }
}
