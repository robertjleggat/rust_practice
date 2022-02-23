use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::str::FromStr;
use std::env;

fn main() {

    let mut number_list = vec![1, 1];

    println!("{:?}", cumulative(&number_list));
    println!("{:?}", mult(&number_list))
}

 // complete
fn sum<T: Add<Output = T> + Copy + Sub<Output = T>>(v: &[T]) -> T {

    let mut total = v[0] - v[0];
    for &item in v {
        total = total + item;
    }
    total

}

// complete
fn mult<T: Mul<Output = T> + Copy + Div<Output = T> + std::str::FromStr>(v: &[T]) -> Vec<T> {

    let mut constant = v[0] / v[0];
    let mut v_f: Vec<T> = Vec::new();
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(element) => constant = constant * match element.parse::<T>(){
            Ok(el) => el,
            Err(el) => constant,
        },
        None => println!("No argument"),
    }

    for &item in v{
        v_f.push(item * constant)
    }
    v_f
}  


// Completed
fn cumulative<T: Add<Output = T> + Copy + Sub<Output = T>>(v: &[T]) -> Vec<T> {
    let mut total = v[0] - v[0];
    let mut v_f: Vec<T> = Vec::new();
    for &item in v {
        total = total + item;
        v_f.push(total)
    }
    v_f
}
