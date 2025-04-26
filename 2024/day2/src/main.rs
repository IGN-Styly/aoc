use std::{fs::File, io::Read, u64};

fn main() {
    let mut count = 0;

    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for l in contents.lines() {
        let mut ret = true;
        let k: Vec<u64> = l.split(" ").map(|f| f.parse().unwrap()).collect();
        let mut k2 = k.clone();
        k2.sort();
        let mut k3 = k2.clone();
        k3.reverse();
        if (k != k2 && k != k3) {
            ret = false;
        }
        for n in 1..k.len() {
            let d = k.get(n - 1).unwrap().abs_diff(k.get(n).unwrap().clone());
            if (d > 3 || d == 0) {
                ret = false;
            }
        }
        if ret {
            count += 1
        };
    }
    println!("Count: {}", count);
}
