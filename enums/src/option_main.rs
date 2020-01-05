fn main() {
    let some_int = Some(5);
    let someother_int = Some(12);
    let some_string = Some("ljhkhguguih");
    let absent_int: Option<i32> = None;

    let sum = some_int.unwrap_or_default() + absent_int.unwrap_or_default();

    println!("some int {:?}", some_int);
    println!("some string {:?}", some_string);
    println!("absent int {:?}", absent_int);
    println!("sum optional ints {}", sum);
}


// #[derive(Debug)]
// enum Option <T> {
//     Some(T),
//     None,
// }



