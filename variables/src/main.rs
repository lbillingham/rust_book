extern crate rand;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is now {}", x);
    println!("");

    let y = 5;
    println!("The value of y is {}", y);
    let y = y + 1;
    println!("The value of y is now {}", y);
    let y = y * 2;
    println!("The value of y is now {}", y);
    println!("");

    let mut spaces = String::from("    ");
    spaces = spaces.len().to_string();
    println!("num spaces is {}", &spaces);
    println!("The value of y is now {}", y);
    println!("");

    let mut z: u8 = 255;
    println!("z is now {}", z);
    z = z + 0;
    println!("z is now {}", z);
    println!("The value of y is now {}", y);
    println!("");

    let a = 2.0;
    let b: f32 = 4.0;
    let res = a * b + (3. * (a + b));
    println!("The value of res is now {}", res);
    println!("");

    let latlon: (f64, f64, &str) = (52.134232, -3.1243346, "wgs84");
    println!("The value of latlons is now {:#?}", latlon);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let mut rng = rand::thread_rng();
    let min_year = 1792;
    let max_year = 2050;
    let year = rng.gen_range(min_year + 1, max_year + 1);
    let month = rng.choose(&months).unwrap();
    let min_day = 1;
    let max_day = num_days(month, year);
    let day = rng.gen_range(min_day + 1, max_day + 1);
    println!("The randomly chosen date is: {}-{}-{}", year, month, day);

    let condition = 2 == 2;
    let number = if condition {
        "5"
    } else {
        "six"
    };

    println!("The value of number is: {}", number);

   let mut counter = 0;
   let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 3;
        }
   };
    println!("The value of counter is: {}, and of result is: {}", counter, result);
    println!("");
    let mut number = 5;
    while number > 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFT OFF !!!1111!");

    let start_count = 5;
    for count in (1..start_count).rev() {
        println!("{}", count);
    }
    println!("LIFT OFF !!!1111!");

}

fn is_leap(year: i32) -> bool {
    if year % 100 == 0 {
        if year % 400 == 0 {
            return true;
        } else {
            return false;
        }
    } else if year % 4 == 0 {
        return true;
    }
    false
}

fn num_days(month: &str, year: i32) -> i32 {
    let have_thirty = ["April", "June", "September", "November"];
    if have_thirty.contains(&month) {
        return 30;
    } else if month == "February" {
        if is_leap(year) {
            return 29;
        } else {
            return 28;
        };
    };
    31
}

