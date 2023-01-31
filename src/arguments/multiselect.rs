use super::common_trait::{Default, Run};
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use std::{io::Result, iter::repeat};

/// Prompt that allows the user to select multiple items from a list of options
#[derive(Debug, Parser)]
pub struct MultiSelect {
    /// Message for the prompt
    #[clap(short, long)]
    message: String,

    /// Makes the prompt cancellable with 'Esc' or 'q'
    #[clap(short, long)]
    cancel: bool,

    /// Makes the prompt return default values as given if --cancel option is present
    #[clap(short = 'd', long = "default", requires = "cancel")]
    return_default: bool,

    /// Returns index of the selected items instead of items itself
    #[clap(short, long)]
    index: bool,

    /// Do not print the selected items on the prompt line
    #[clap(long)]
    no_inline: bool,

    /// Specify numbers of items that will be selected by default
    #[clap(short, long)]
    selected: Vec<usize>,

    /// Items that can be selected
    items: Vec<String>,
}

impl Run<Vec<String>, std::io::Error> for MultiSelect {
    fn run(&self) -> Result<Vec<String>> {
        let item_len = self.items.len();

        if item_len == 0 {
            return Ok(vec![]);
        }

        let theme = ColorfulTheme {
            inline_selections: !self.no_inline,
            ..ColorfulTheme::default()
        };

        let mut input = dialoguer::MultiSelect::with_theme(&theme);
        let mut defaults = vec![];

        defaults.extend(repeat(false).take(item_len));

        for i in &self.selected {
            if *i > item_len {
                continue;
            }

            defaults[i - 1] = true;
        }

        input
            .with_prompt(&self.message)
            .clear(true)
            .items(&self.items)
            .defaults(&defaults);

        let ret = if self.cancel {
            input.interact_opt()?
        } else {
            Some(input.interact()?)
        };

        let value = match ret {
            Some(value) => value
                .iter()
                .map(|i| self.items[*i].clone())
                .collect::<Vec<String>>(),
            None => vec![],
        };

       
        Ok(value)
    }
}

impl Default for MultiSelect {
    fn default(message: &str, can_be_nullable: Option<bool>, items: Option<Vec<String>>) -> Self {
        Self {
            message: message.to_string(),
            return_default: false,
            no_inline: false,
            cancel: can_be_nullable.unwrap_or(false),
            index: false,
            selected: vec![],
            items: items.unwrap_or_default(),
        }
    }
}
