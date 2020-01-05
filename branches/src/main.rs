fn main() {
    let number = -422;
    let even = if number%2 == 0 { true } else { false };
    if even
    {
        println!("number ({}) is even ", number);
    } else {
        println!("numbier ({}) is odd", number);
    }
    println!("###########################");
    let condition = false;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    println!("###########################");
    let mut counter = 0;
    println!("counter originally {}", counter);
    let result = loop {
        counter += 1;

        if counter >= 10 {
            break counter * 3;
        }
    };
    println!("counter is now {}", counter);
    println!("... and result is {}", result);

    println!("###########################");
    let mut num = 10;
    while num > 0 {
        println!("num is {}", num);
        num -= 1;
    }

    println!("###########################");
    let while_arr = [10, 20, 30, 40, 50];
    let mut while_idx = 0;
    while while_idx < 5 {
        println!("the {}th element is {}", while_idx + 1, while_arr[while_idx]);
        while_idx += 1;
    }

    println!("###########################");
    let for_arr = [1, 10, 100, 1000, 10000, 100_000];
    for elem in for_arr.iter() {
        println!("elem is {}", elem);
    }

    println!("###########################");
    for cdown in (1..11).rev() {
        println!("countdown (with for) at {}", cdown);
    }
    println!("LIFTOFF (with for range)");
}
