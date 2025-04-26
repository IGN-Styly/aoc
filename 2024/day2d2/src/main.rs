use std::{fs::File, io::Read, u64};

fn isSafe(data: Vec<u64>) -> bool {
    let mut ret = true;
    let k: Vec<u64> = data;
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
    ret
}

fn main() {
    let mut count = 0;

    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for l in contents.lines() {
        let mut ret = true;
        let k: Vec<u64> = l.split(" ").map(|f| f.parse().unwrap()).collect();
        ret = isSafe(k.clone());
        for n in 0..k.len() {
            let mut nk = k.clone();
            nk.remove(n);
            ret = isSafe(nk) | ret;
        }
        if ret {
            count += 1
        };
    }
    println!("Count: {}", count);
}
