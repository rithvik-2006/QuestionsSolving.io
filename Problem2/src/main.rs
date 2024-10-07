fn main() {
    abbrev_name("Sam Harris");
    println!("{}", abbrev_name("Sam Harris"));
}

fn abbrev_name(name: &str) -> String {
    let mut parts = name.split_whitespace();
    let first_initial = parts.next().unwrap().chars().next().unwrap().to_uppercase().to_string();
    let last_initial = parts.next().unwrap().chars().next().unwrap().to_uppercase().to_string();
    println!("{}, {}", first_initial, last_initial);
    format!("{}.{}", first_initial, last_initial)
}
