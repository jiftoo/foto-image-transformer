use regex::Regex;

mod proc;

// Process cli arguments
fn main() {
    // proc::execute(std::env::args().skip(1).collect());

    let regex = Regex::new(r"([0-9]{1,3})x([0-9]{1,3})").unwrap();
    for cap in regex.captures("640x480").unwrap().iter().skip(1) {
        println!("{:?}", cap.unwrap().as_str());
    }
}
