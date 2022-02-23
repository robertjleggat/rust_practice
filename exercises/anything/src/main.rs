use std::env;
use std::any::type_name;

fn main() {

    
}

fn variable_type<T: std::str::FromStr>() -> &'static str{
    let args: Vec<String> = env::args().collect();
    let mut var: T;
    match args.get(1) {
        Some(element) => match element.parse() {
            Ok(el) => var = el,
            Err(el) => var = element,
        },
        None => (),
    };
    type_of(var)
}

fn type_of<T>(_: T) -> &'static str{
    std::any::type_name::<T>()
}