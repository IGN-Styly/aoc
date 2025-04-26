use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let mut count: u64 = 0;
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    for l in contents.lines() {
        let matches: Vec<&str> = re.find_iter(l).map(|mat| mat.as_str()).collect();

        for m in matches {
            let k: Vec<u64> = m
                .strip_prefix("mul(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(",")
                .map(|f| f.parse().unwrap())
                .collect();
            count += k.iter().product::<u64>();
        }
    }
    println!("count: {}", count);
}
