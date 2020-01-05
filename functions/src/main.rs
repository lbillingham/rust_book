fn main() {
    println!("Hello, world!");
    another_function(43, 2);
    println!("Hello, from after the other function get's called");
    println!("value of twelve() is {}", twelve());
    println!("##########################");
    let foo = 41;
    println!("{} was the value of foo", foo);
    println!("after incrementing it is {}", increment(foo));
}

fn another_function(x: i32, y: i32) {
    println!("the value of x is {}", x*y);
}

fn increment(x :i32) -> i32 {
    x + 1
}

fn twelve() -> i32 {
    4 * 3
}
