use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(element) => print!("{} ", element),
        None => println!("No argument"),
    }

    let mut i = 1;
    loop {
        match args.get(i) {
            Some(element) => print!("{} ", element),
            None => break,
        }
        i = i + 1;
    }
}
