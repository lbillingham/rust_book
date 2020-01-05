fn main() {
    let p = Coin::Penny;
    let d = Coin::Dollar;
        // let ccmess = Message::ChangeColor(-21, 84, 42);
    let pval = value_in_cents(&p);
    println!("value in cents of {:?} is {}", p, pval);
    println!("value in cents of {:?} is {}", d, value_in_cents(&d));

    let vq = Coin::Quarter(UsState::Vermont);
    println!("value in cents of {:?} is {}", vq, value_in_cents(&vq));
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter form {:?}", state);
            25
        },
        Coin::Dollar => 100,
    }
}


#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
    Dollar,
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}



