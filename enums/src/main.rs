fn main() {


    let five = Some(5);
    let six = incr(five);
    let none = incr(None);
    println!("five {:?}", five);
    println!("six {:?}", incr(incr(six)));
    println!("none {:?}", none);
    println!("");
    println!("");
    println!("");
    println!("");
    printyer(7);
    println!("---");
    printyer(5);
    println!("---");
    printyer(4);
    printyer(5u8);

}

fn incr(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(1+i),
        _ => None,
    }
}

fn printyer(x: u8) {
    match x {
        1 => println!("one ha ha ha"),
        3 => {
            println!("THREEE...");
            printyer(1);
        },
        5 => {
            println!("FiVE3333!!!");
            printyer(3);
        },
        7 => println!("Seben"),
        _ => ()
    }
}