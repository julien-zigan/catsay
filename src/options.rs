use clap::Parser;

#[derive(Parser)]
pub struct Options {
    #[clap(default_value = "Meow!")]
    /// What the cat says
    pub message: String,

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    pub dead: bool,

    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    pub catfile: Option<std::path::PathBuf>,

    #[clap(short = 'i', long = "stdin")]
    /// Read the message from stdin instead of the argument
    pub stdin: bool,
}

impl Options {
    pub fn new() -> Self {
        Options::parse()
    }
}