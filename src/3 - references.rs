fn main() {
    references()
}


fn references() {
    let s1 = String::from("Rust!");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);


    let mut s1 = String::from("Rust!");
    change(&mut s1);
    println!("{} new s1 ", s1)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}