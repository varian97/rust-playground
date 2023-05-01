mod vector_utils;
mod string_utils;

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

    let mut s1 = String::from("First");
    let mut s2 = String::from("apply");

    println!("String s1: {s1} and s2: {s2}");

    string_utils::convert_to_pig_latin(&mut s1);
    string_utils::convert_to_pig_latin(&mut s2);
    println!("After pig latin: String s1: {s1} and s2: {s2}");
}
