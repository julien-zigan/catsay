fn main() {
    let message = std::env::args();

    for argument in message {
        println!("{}", argument);
    }
}
