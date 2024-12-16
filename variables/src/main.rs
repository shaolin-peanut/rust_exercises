fn main() {
    let mut x = 5;
    const SPECIFIC: u32 = 60 * 60 * 3;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("The value of specific is: {SPECIFIC}");

    let first = String::from("Ferris");
    let full = add_suffix(first.clone());
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
