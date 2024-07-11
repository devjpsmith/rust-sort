/*
    "Unstable" sort, but useful due to it's complexity
    Complexity: O(n log n)
*/

use std::collections::BinaryHeap;

// uh, yeah... I totally fuckin cheated cuz it's 10:15pm and I don't want to roll my own binary search tree
pub fn sort(mut vec: Vec<i32>) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    let mut sorted = Vec::new();
    for i in vec {
        heap.push(i)
    }

    for _i in 0..heap.len() {
        sorted.push(heap.pop().unwrap())
    }

    sorted.reverse();
    sorted
}