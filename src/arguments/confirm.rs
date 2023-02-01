use clap::Parser;
use dialoguer::theme::ColorfulTheme;

use std::io::Result;

/// Prompt that returns `true` or `false` (as strings)
#[derive(Debug, Parser, Default)]
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
}

impl Confirm {
    pub fn _run(&self) -> Result<()> {
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
        } else {
            println!("false");
        }

        Ok(())
    }
}
