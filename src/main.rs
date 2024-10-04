mod options;

use crate::options::Options;
use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Read};



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let options = Options::new();

    let mut message = String::new();

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprint!("A cat shouldn't bark!")
    }

    let eye = if options.dead {
        "x".red().bold()
    } else {
        "o".bold()
    };

    println!();
    println!("{}", message.red().bold().on_bright_yellow());

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read file {:?}", path))?;
            let eye = format!("{}", eye);
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", &cat_picture);
        }
        None => {
            println!("  \\");
            println!("   \\");
            println!("     /\\_/\\");
            println!("    ( {0} {0} )", eye);
            println!("    =( I )=");
        }
    }
    println!();

    Ok(())
}
