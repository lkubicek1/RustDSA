use rand::prelude::*;
use rand::Rng;

pub fn generate_random_vec(size: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(rng.gen_range(0..100_000));
    }
    vec
}

pub fn generate_sorted_vec(size: usize) -> Vec<i32> {
    let mut vec = Vec::with_capacity(size);
    for i in 0..size {
        vec.push(i as i32);
    }
    vec
}

pub fn generate_reverse_sorted_vec(size: usize) -> Vec<i32> {
    let mut vec = Vec::with_capacity(size);
    for i in 0..size {
        vec.push((size - i) as i32);
    }
    vec
}
