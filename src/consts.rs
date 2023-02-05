use crate::gits::behavior::UserInput;

pub const WELCOME_MESSAGE: &str = "welcome to GITTY.";
pub const CHOOSE_COMMAND: &str = "Choose the command you want to execute:";
pub const OPTION_MESSAGES: &[UserInput] = &[
    UserInput::Clone("clone the project"),
    UserInput::HardSync("sync the existing project with remote repo (Hard reset)"),
    UserInput::SoftSync("sync the existing project with remote repo (Soft reset)"),
    UserInput::Purge("delete unnecessary branches"),
    UserInput::Diff("delete the branches that's not on remote"),
];
pub const CHOOSE_BRANCHES: &str = "Choose the branches to pull:";
pub const DEFAULT_BRANCH: &str = "default branch will be pulled automatically";
pub const CHOOSE_DELETE_BRANCHES: &str = "Choose the branches to delete:";
pub const PROPER_URL_WARNING: &str = "Please enter a proper URL";
