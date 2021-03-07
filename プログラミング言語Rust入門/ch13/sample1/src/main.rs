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

    let n = 10;
    println!("通常    {}", n);
    println!("桁揃え {:4}", n);
    println!("10進数 {:04}", n);
    println!("16進数 {:02X}", n);
    println!(" 2進数 {:08b}", n);

    let f = 10.234;
    println!("{} ", f);
    println!("{:e} ", f);
    println!("{:E} ", f);
    println!("{:.2}", f);
    let f = 0.0234;
    println!("{} ", f);
    println!("{:e} ", f);
    println!("{:E} ", f);
    println!("{:.2}", f);

    println!("hello, `{:8}` world.", "rust");
    println!("hello, `{:<8}` world.", "rust");
    println!("hello, `{:>8}` world.", "rust");
    println!("hello, `{:^8}` world.", "rust");
    println!("hello, `{:8}` world.", 123);
    println!("hello, `{:<8}` world.", 123);
    println!("hello, `{:>8}` world.", 123);
    println!("hello, `{:^8}` world.", 123);
}
