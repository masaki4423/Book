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

    println!("a is {}, b is {}", 100, "test");
    println!("a is {a}, b is {b}", a=100, b="test");

    let n = 10;
    println!("10進数 {}", n);
    println!("16進数 {:x}", n);
    println!("16進数 {:X}", n);
    println!(" 8進数 {:o}", n);
    println!(" 2進数 {:b}", n);
}
