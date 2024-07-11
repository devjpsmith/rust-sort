/*
    Efficient and commonly used
    Complexity: O(n log n) usually; O(n^2) with a bad pivot
*/

fn partition(mut vec: &mut Vec<i32>, low: isize, high: isize) -> isize {
    let pivot = vec[high as usize];
    let mut p_index = low - 1;

    for i in low..high {
        if vec[i as usize] < pivot {
            // increment pivot index
            p_index += 1;
            vec.swap(p_index as usize, i as usize);
        }
    }

    p_index += 1;
    vec.swap(p_index as usize, high as usize);

    p_index
}

fn quick_sort(mut vec: &mut Vec<i32>, low: isize, high: isize) -> Vec<i32> {
    if low < high {
        let p_index = partition(vec, low, high);

        quick_sort(vec, low, p_index - 1);
        quick_sort(vec, p_index + 1, high);
    }

    vec.to_vec()
}

pub fn sort(mut vec: Vec<i32>) -> Vec<i32> {
    let high = vec.len() as isize - 1;
    vec = quick_sort(&mut vec, 0, high);

    vec
}