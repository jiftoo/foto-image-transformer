use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("{}", args[0])
        }
        _ => {
            eprintln!("No arguments provided");
        }
    }
}
