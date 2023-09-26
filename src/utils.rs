use std::cmp::Reverse;
use std::fmt::Debug;
use ordered_float::NotNan;
use std::fs::File;
use std::io::{Write, Result};

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

type MinNonNan = Reverse<NotNan<f64>>;
pub fn from_float(float: f64) -> Reverse<NotNan<f64>> {
    Reverse(NotNan::new(float).unwrap())
}

pub fn print<T: Debug>(any: T) {
    println!("{:?}", any);
}

pub fn export_txt<T: Debug>(any: T, file_name: String) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(format!("{:?}", any).as_bytes()).expect("Message could not be written");
}
