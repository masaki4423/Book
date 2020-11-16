fn main() {
    let s = "hello rust world.";
    let a = &s[0..1];
    println!("a is {}", a);
    let a = &s[0..5];
    println!("a is {}", a);
    let a = &s[..5];
    println!("a is {}", a);

    let s = "hello rust world.";
    let a = &s[6..10];
    println!("a is {}", a);
    let a = &s[6..(6+4)];
    println!("a is {}", a);

    let s = "hello rust world.";
    let a = &s[11..];
    println!("a is {}", a);
    let n = s.len();
    let a = &s[11..n];
    println!("a is {}", a);

    // let s = "hello rust world.";
    // let a = &s[11..20];
    // println!("a is {}", a);

    let s = "hello rust world.";
    let a = &s[..];
    println!("a is {}", a);
    let n = s.len();
    let a = &s[0..n];
    println!("a is {}", a);
}
