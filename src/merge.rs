/*
    Stable sort for large datasets
    Complexity: O(n log n)
*/

fn merge(mut left: &Vec<i32>, mut right: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i += 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged
}

fn merge_sort(arr: &Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        arr.to_vec()
    } else {
        let s = arr.len() / 2;
        let left = merge_sort(&arr[0..s].to_vec());
        let right = merge_sort(&arr[s..].to_vec());
        let merged  = merge(&left, & right);

        merged
    }
}

pub fn sort(arr: Vec<i32>) -> Vec<i32> {
    merge_sort(&arr)
}