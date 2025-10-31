fn main() {
    let str = String::from("Hello, world!");
    let len = get_string_length(str);
    println!("Length of string {}",len);
}

fn get_string_length(s: String) -> usize {
    return s.chars().count();
}

