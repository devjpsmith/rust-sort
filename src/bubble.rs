/*
    Easy to understand an implement, but not really useful due to inefficiency
    Complexity: O(n^2)
*/

pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    for _i in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr.get(j) > arr.get(j + 1) {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}