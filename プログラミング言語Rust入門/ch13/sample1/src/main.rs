fn main() {
    println!("hello rust world.");
    println!("hello {} world.", "rust");

    println!("number is {}.", 10);
    println!("float is {}.", 10.234);

    println!("tuple is {:?}.", ("masuda", 50));

    let n = Option::<i32>::Some(10);
    println!("option is {:?}.", n);
    let n = Option::<i32>::None;
    println!("option is {:?}.", n);

    let n = Option::<i32>::Some(10);
    println!("option is {}.", n.unwrap());
}
