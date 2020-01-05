fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);
    let rect2 = Rectangle{width:30, height:50};

    println!(
        "The area0 of the rectangle is {} square pixels.",
        area0(width1, height1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );
    println!(
        "The area of the rectangle ({:?}) is {} square pixels.",
        rect2, area2(&rect2)
    );
    println!(
        "The  {:?}.area is {} square pixels.",
        rect2, &rect2.area()
    );

    let rect3 = Rectangle { width: 10, height: 40 };
    let rect4 = Rectangle { width: 60, height: 45 };


    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    let sqr = Rectangle::square(41);
    println!("Square {:?}'s area is {}; could it hold rect3?: {}", sqr, sqr.area(), sqr.can_hold(&rect3));

}

fn area0(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}

fn area2(reccy: &Rectangle) -> u32 {
    reccy.width * reccy.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}