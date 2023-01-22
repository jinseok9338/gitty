use clap::Parser;



#[derive(Parser, Debug)]
#[command(author="Jinseok Seo", version="1.0.0", about, long_about = None)]
pub struct Args {
   /// Name of the person to greet
   //not required
   #[arg(short, long, default_value = "")]
   pub name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   pub count: u8,

    /// URL for cloning the repository
    #[arg(short, long, default_value = "")]
    pub url: String,


    /// Directory to clone the repository
    #[arg(short, long)]
    pub directory: String,

}




