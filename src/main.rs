use rand::{Rng};
use std::time::SystemTime as SystemTime;
mod bubble;
mod selection;
mod insertion;
mod merge;
mod quick;
mod heap;

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 1..50 {
        vec.push((rng.gen::<f64>() * 100.0).floor() as i32)
    }

    let start = SystemTime::now();
    println!("unsorted --> {:?}", vec);
    vec = heap::sort(vec);
    println!("sorted ----> {:?}", vec);
    let duration = SystemTime::now().duration_since(start);
    println!("sort time: {:?}", duration);
}
