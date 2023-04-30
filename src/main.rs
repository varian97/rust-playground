mod vector_utils;

fn main() {
    let mut v = vec![10, 20, 10, 24, 21, 10, 11, 22, 15, 25];
    println!("The list is {:#?}", v);

    let len = v.len() - 1;
    vector_utils::mergesort(&mut v, 0, len);
    println!("The sorted list is {:#?}", v);

    let mode = vector_utils::find_mode(&v);
    println!("The mode is {mode}");

    let median = vector_utils::find_median(&v);
    println!("The median is {median}");
}
