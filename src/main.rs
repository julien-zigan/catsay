use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What the cat says
    message: String,

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::parse();
    let message = options.message;

    if message.to_lowercase() == "woof" {
        eprint!("A cat shouldn't bark!")
    }

    let eye= if options.dead {"x"} else {"o"};

    println!();
    println!("{}", message.yellow().bold().on_red());
    println!("  \\");
    println!("   \\");
    println!("     /\\_/\\");
    println!("    ( {0} {0} )", eye);
    println!("    =( I )=");
    println!();
}
