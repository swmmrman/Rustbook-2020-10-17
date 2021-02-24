use std::{process,env,collections::HashMap};
use std::convert::TryInto;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut defaults = vec![32, 18, 11, 18, 6, 66, 56, 5, 29, 61, 95, 61, 22, 52, 90, 52, 65, 13, 87, 12, 21, 86, 5, 15, 4, 77, 78, 62, 66, 64, 84, 18, 46, 90, 24, 22, 41, 37, 55, 71, 68, 13, 30, 27, 85, 5, 58, 54, 80, 19, 88, 97, 25, 75, 68, 60, 11, 33, 17, 7, 51, 74, 53, 16, 44, 70, 74, 14, 86, 50, 27, 5, 82, 91, 11, 63, 52, 61, 15, 16, 92, 73, 7, 93, 51, 84, 87, 43, 12, 97, 2, 32, 73, 91, 13, 20, 3, 57, 49, 87];
    let mut integers = vec![];
    let mut random_on = false;
    match args.len() {
        1 => {
            integers.append(&mut defaults);
        },
        3=> {
            if args[1] == "-r"{
                let count = match args[2].parse::<i32>() {
                    Ok(i) => i,
                    Err(_e) => {
                        println!("-r must be followed by the # of integers you want.");
                        process::exit(1)
                    },
                };
                random_on = true;
                let mut rng = rand::thread_rng();
                for i in 0..count{
                    integers.push(rng.gen_range(1..100));
                }
            }
        }
        _ => {
            let mut index = 0;
            for arg in args{
                if index == 0 {
                    index +=1;
                    continue;
                }
                let value = match arg.parse::<i32>() {
                    Ok(i) => i,
                    Err(_e) => {
                        println!("Non integer in list");
                        process::exit(1);
                    },
                };
                integers.push(value);
            }
        }
    }
    let mut len = 0;
    let mut sum = 0;
    let mut values = HashMap::new();
    for i in integers.clone(){
        len += 1;
        sum += i;
        let count = values.entry(i).or_insert(0);
        *count += 1;
    }
    let mut ordered = integers.to_vec();
    ordered.sort();
    if random_on {
        println!("The integers are:{:?}\n\n", integers);
    }
    let mean = sum / len;
    let mid: usize = (len/2).try_into().unwrap();
    let median = ordered[mid];
    let mut mode = 0;
    let mut mode_count = 0;
    for (k, v) in &values{
        if v > &mode_count{
          mode = *k;
          mode_count = *v;
      }
    }
    println!("Sum = {}\nLength = {}\nMean = {}\nMedian = {}\nMode = {}", sum, len, mean, median, mode);
}
