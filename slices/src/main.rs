fn main() {
    let phrase = String::from("sadgjaosdjf fsfasdf, world!");
    let first = first_word(&phrase);
    println!("'{}' is the first word of '{}' ", first, phrase)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
        &s[..]
}
