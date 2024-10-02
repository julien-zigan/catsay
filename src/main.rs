use clap::Parser;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What the cat says
    message: String,
}

fn main() {
    let options = Options::parse();
    let message = options.message;

    println!();
    println!("{}", message);
    println!("  \\");
    println!("   \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    =( I )=");
    println!();
}
