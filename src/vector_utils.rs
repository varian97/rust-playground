use std::collections::HashMap;

pub fn mergesort(list: &mut [i32], start: usize, end: usize) {
    if start < end {
        let middle = start + (end - start) / 2;
        mergesort(list, start, middle);
        mergesort(list, middle + 1, end);
        merge(list, start, middle, end);
    }
    return;
}

fn merge(list: &mut [i32], left_index: usize, mid_index: usize, right_index: usize) {
    let mut temp_left = Vec::new();
    let mut temp_right = Vec::new();

    for i in left_index..=mid_index {
        temp_left.push(list[i])
    }
    for i in mid_index+1..=right_index {
        temp_right.push(list[i]);
    }

    let mut k = left_index;

    let mut i = 0;
    let mut j = 0;

    while i < temp_left.len() && j < temp_right.len() {
        if temp_left[i] <= temp_right[j] {
            list[k] = temp_left[i];
            i += 1;
        } else {
            list[k] = temp_right[j];
            j += 1;
        }
        k += 1;
    }
    while i < temp_left.len() {
        list[k] = temp_left[i];
        i += 1;
        k += 1;
    }
    while j < temp_right.len() {
        list[k] = temp_right[j];
        j += 1;
        k += 1;
    }
}

pub fn find_mode(list: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for item in list {
        let val = occurrences.entry(item).or_insert(0);
        *val += 1;
    }

    let mut mode = list[0];
    let mut mode_count = occurrences.get(&mode).copied().unwrap_or(0);

    for item in list {
        let count = occurrences.get(item).copied().unwrap_or(0);
        if count > mode_count {
            mode = *item;
            mode_count = count;
        }
    }

    mode
}

pub fn find_median(list: &[i32]) -> f32 {
    let len = list.len();

    if len % 2 == 0 {
        let bmid = (len - 1) / 2;
        let tmid = bmid + 1;

        let total = (list[bmid] + list[tmid]) as f32;
        return total / 2.0;
    }

    list[(len - 1) / 2] as f32
}
