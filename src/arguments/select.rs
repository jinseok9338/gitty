
use dialoguer::theme::ColorfulTheme;
use std::fmt::Debug;
use super::common_trait::{Default, Run};

use std::{error::Error};


/// Prompt that allows the user to select from a list of options
#[derive(Debug)]
pub struct Select<T:Clone +Send + Sync + Debug>{
    /// Message for the prompt
   
    message: String,

    /// Makes the prompt cancellable with 'Esc' or 'q'
    
    cancel: bool,

    /// Makes the prompt return default order as given if --cancel option is present
 
    return_default: bool,

    /// Returns index of the selected item instead of item itself

    index: bool,

    /// Specify number of the item that will be selected by default
  
    selected: Option<usize>,

   
    items: Vec<T>,
}



impl<T:Clone +Send + Sync +Debug > Run<T, Box<dyn Error>> for Select<T> {
    fn run(&self) -> Result<T, Box<dyn Error>> {
        let item_len = self.items.len();

        if item_len == 0 {
            // return error

            return Err("No items to select from".into());
        }

        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Select::with_theme(&theme);
        //convert Vec<T> to Vec<String>
        let mut items = vec![];
        for item in &self.items {
            items.push(format!("{:?}", item));
        }

        input
            .with_prompt(&self.message)
            .clear(true)
            .items(&items);

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

impl<T:Clone +Send + Sync+Debug > Default<T> for Select<T> {
    fn default(message: &str, can_be_nullable: Option<bool>, items: std::option::Option<Vec<T>>) -> Self {
        Self {
            message: message.to_string(),
            return_default: false,
            cancel: can_be_nullable.unwrap_or(false),
            index: false,
            selected: Some(1),
            items: items.unwrap(),
        }
    }
}
