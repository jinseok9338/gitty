use clap::Parser;
use dialoguer::theme::ColorfulTheme;

use super::common_trait::{Default, Run};

use std::error::Error;

/// Prompt that allows the user to select from a list of options
#[derive(Debug, Parser)]
pub struct Select {
    /// Message for the prompt
    #[clap(short, long)]
    message: String,

    /// Makes the prompt cancellable with 'Esc' or 'q'
    #[clap(short, long)]
    cancel: bool,

    /// Makes the prompt return default order as given if --cancel option is present
    #[clap(short = 'd', long = "default", requires_all = &["cancel", "selected"])]
    return_default: bool,

    /// Returns index of the selected item instead of item itself
    #[clap(short, long)]
    index: bool,

    /// Specify number of the item that will be selected by default
    #[clap(short, long)]
    selected: Option<usize>,

    /// Items that can be selected
    items: Vec<String>,
}

impl Run<String, Box<dyn Error>> for Select {
    fn run(&self) -> Result<String, Box<dyn Error>> {
        let item_len = self.items.len();

        if item_len == 0 {
            // return error

            return Err("No items to select from".into());
        }

        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Select::with_theme(&theme);

        input
            .with_prompt(&self.message)
            .clear(true)
            .items(&self.items);

        let selected = self.selected.map(|i| i - 1);

        if let Some(index) = selected {
            input.default(index);
        }

        let ret = if self.cancel {
            input.interact_opt()?
        } else {
            Some(input.interact()?)
        };

        let value = match ret {
            Some(value) => value,
            None if self.return_default && selected.is_some() => selected.unwrap(),
            None => std::process::exit(1),
        };

        let result = self.items[value].clone();

        Ok(result)
    }
}

impl Default for Select {
    fn default(message: &str, can_be_nullable: Option<bool>, items: Option<Vec<String>>) -> Self {
        Self {
            message: message.to_string(),
            return_default: false,

            cancel: can_be_nullable.unwrap_or(false),
            index: false,
            selected: Some(1),
            items: items.unwrap_or(vec![]),
        }
    }
}
