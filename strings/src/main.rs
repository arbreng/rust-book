fn main() {
    let mut h = String::from("Hello");

    println!("The length of {} is: {}", &h, length(&h));
    change(&mut h);
    println!("The length of {} is: {}", &h, length(&h));
    println!("The first word is {}", first_word(&h));
}

fn length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}