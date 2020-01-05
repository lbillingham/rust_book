fn main() -> () {
    let s = String::from("Hello the world");
    let w1 = first_word(&s);
    println!("the first word in '{}' is '{}'", s, w1);
    let w2 = nth_word(&s, 3);
    println!("the 2nd word in '{}' is '{:?}'", s, w2);
}

fn first_word(some_string: &String) -> &str {
    let bytes = some_string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[0..i]
        }
    }
    some_string
}

fn nth_word(some_string: &String, n: i32) -> &str {
    let bytes = some_string.as_bytes();
    let mut start_index = 0;
    let mut end_index = some_string.len();
    let mut words_found = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start_index == 0 {
                println!("")
            } else {
                end_index = i;
                words_found +=1 ;
            }
        }
        if words_found == n {
            break;
        }
    }
    &some_string[start_index..end_index]
}