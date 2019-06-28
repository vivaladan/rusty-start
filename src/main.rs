#![allow(dead_code)]

mod averages;
mod piggy;

fn main() {
    //averages::calc_averages();
    println!("{}", piggy::to_pig_latin("What time is it".to_string()));
}