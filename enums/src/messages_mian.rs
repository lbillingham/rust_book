fn main() {

    let wmess = Message::Write(String::from("aksjsdjljsdf"));
    println!("calling  {:?}", wmess);
    wmess.call();

    let qmess = Message::Quit;
    println!("calling  {:?}", qmess);
    qmess.call();

    let mmess = Message::Move{x: 12, y: 17};
    println!("calling  {:?}", mmess);
    mmess.call();

    let ccmess = Message::ChangeColor(-21, 84, 42);
    println!("calling  {:?}", ccmess);
    ccmess.call();
}


#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message {:?} has been called", self)
    }
}

