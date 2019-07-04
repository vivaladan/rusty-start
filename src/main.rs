#![allow(dead_code)]
#![allow(unused_imports)]

mod averages;
mod piggy;
mod employees;

fn main() {
    //averages::calc_averages();
    //println!("{}", piggy::to_pig_latin("What time is it".to_string()));
    employees::start_repl();
}