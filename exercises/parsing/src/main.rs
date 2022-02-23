use std::fs::File;
use std::fs;
use std::io;
use std::io::Read;
use std::collections::HashMap;

fn main() {

    let mut s = fs::read_to_string("phonebook.txt").unwrap();
    let mut split = s.split("\n");
    let vec: Vec<&str> = split.collect();
    let mut book = HashMap::new();
    for item in &vec {
        let entry: Vec<&str> = item.split(";").collect();
        book.insert(entry[0], entry[1].trim());
    }
    println!("{:?}", book);

    loop {
        println!("Please enter a name:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: String =String::from(guess.trim());
        let res = book.entry(&guess).or_insert("No result");
        println!("{}", res);
    }
}
