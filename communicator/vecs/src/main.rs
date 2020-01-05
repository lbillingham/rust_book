fn main() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3 = Vec::new();
    v3.push(123);
    v3.push(12);
    v3.push(1);
    println!("Hello, world!, {:?}... {:?}", v2, v3);

    let v4 = vec![1,2,3,4,5,6,7,8];
    let third: &i32 = &v4[2];

    println!("third value of {:?} is {}", v4, third);

    let v_index = 2;
    match v4.get(v_index) {
        Some(_) => {println!("Reachable elem @ index {}, value is {:?}", v_index, v4.get(v_index));},
        None => {println!("unreachable element at index {}", v_index    );}
    }
}
