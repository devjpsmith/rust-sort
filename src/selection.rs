/*
    Easy to understand an implement, but inefficient on large datasets
    Complexity: O(n^2)
*/

pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr.get(j) < arr.get(min_index) {
                min_index = j;
            }
        }
        arr.swap(min_index, i);
    }
    arr
}