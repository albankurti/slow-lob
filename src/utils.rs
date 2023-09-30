use std::cmp::Reverse;
use std::fmt::Debug;
use ordered_float::NotNan;
use std::fs::File;
use std::env;
use std::io::{Write, self, BufRead};
use crate::orderbook::WrappedReverse;

pub fn merge(vec1: Vec<usize>, vec2: Vec<usize>) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::with_capacity(vec1.len() + vec2.len());

    let mut a_pos: usize = 0;
    let mut b_pos: usize = 0;
    for _i in 0..vec.capacity(){
        if a_pos >= vec1.len(){
            vec.extend(&vec2[b_pos..vec2.len()]);
            break;
        }
        if b_pos >= vec2.len(){
            vec.extend(&vec1[a_pos..vec1.len()]);
            break;
        }
        if vec1[a_pos] <= vec2[b_pos] {
            vec.push(vec1[a_pos]);
            a_pos += 1;
        }
        else{
            vec.push(vec2[b_pos]);
            b_pos += 1;
        }
    }
    vec
}

pub fn from_float(float: f64) -> WrappedReverse {
    WrappedReverse(Reverse(NotNan::new(float).unwrap()))
}

pub fn print<T: Debug>(any: T) {
    println!("{:?}", any);
}

pub fn export_txt<T: Debug>(any: T, file_name: String) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(format!("{:?}", any).as_bytes()).expect("Message could not be written");
}

pub fn read_keys(file_name: String){
    let file = File::open(file_name).expect("Could not open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(key) = line {
            let parts: Vec<&str> = key.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0];
                let value = parts[1];
                env::set_var(key, value);
            }
        }
    }
}