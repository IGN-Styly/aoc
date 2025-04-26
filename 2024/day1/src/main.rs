use std::{fs::File, io::Read, u64};

fn main() {
    let mut count = 0;
    let mut l1: Vec<u64> = Vec::new();
    let mut l2: Vec<u64> = Vec::new();
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for c in contents.lines() {
        let k: Vec<u64> = c.split("   ").map(|f| f.parse().unwrap()).collect();
        l1.push(k.get(0).unwrap().clone());
        l2.push(k.get(1).unwrap().clone());
    }
    l1.sort();
    l2.sort();
    for it in l1.iter().zip(l2) {
        let (n1, n2) = it;
        count += n1.abs_diff(n2);
    }
    println!("Count: {}", count);
}
