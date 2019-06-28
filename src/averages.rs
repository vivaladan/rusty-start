use std::collections::HashMap;

pub fn calc_averages() {
    let range = vec![13, 18, 13, 14, 13, 16, 14, 21, 13];
    // mean 15
    // median 14
    // mode 13

    let mean = calc_mean(&range);
    println!("Mean:   {}", mean);

    let median = calc_median(&range);
    println!("Median: {}", median);

    let mode = calc_mode(&range);
    println!("Mode:   {}", mode);
}

fn calc_mean(range: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0; 
    for number in range {
        sum += number;
    }
    sum / range.len() as i32
}

fn calc_median(range: &Vec<i32>) -> f64 {
    let mut sorted = range.clone();
    sorted.sort();
    let len = sorted.len();
    if len % 2 == 0 {
        let idx = len / 2;
        let lower_mid = sorted[idx];
        let upper_mid = sorted[idx + 1];
        let sum_of_mids = lower_mid + upper_mid;
        sum_of_mids as f64 / 2.0
    }
    else {
        let idx = len / 2;
        sorted[idx] as f64
    }
}

fn calc_mode(range: &Vec<i32>) -> i32 {
    let mut number_counts = HashMap::new();
    for number in range {
        let count = number_counts.entry(number).or_insert(0);
        *count += 1;
    }
    let mut highest_count = 0;
    let mut mode = 0;
    for (number, count) in number_counts {
        if count > highest_count {
            highest_count = count;
            mode = *number;
        }
    }
    mode
}