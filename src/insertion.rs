/*
    Effective for small datasets or nearly sorted data
*/

pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 1..arr.len() {
        // j needs to be isize to handle going less than zero in the while loop
        let mut j: isize = (i - 1) as isize;
        let val = arr[i];

        while j >= 0 && arr[j as usize] > val {
            arr[j as usize + 1] = arr[j as usize];
            j = j - 1;
        }

        arr[(j + 1) as usize] = val;
    }

    arr
}