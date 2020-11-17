fn main() {
    let a = 20 + 10;
    println!("a is {}", a);
    let a = 20 - 10;
    println!("a is {}", a);
    let a = 20 * 10;
    println!("a is {}", a);
    let a = 20 / 10;
    println!("a is {}", a);
    let a = 20 % 3;
    println!("a is {}", a);

    let a = 10;
    let b = 3;
    let ans = a / b;
    println!("a / b is {}", ans);
    let a = 10.0;
    let b = 3.0;
    let ans = a / b;
    println!("a / b is {}", ans);

    let mut a = 10;
    a += 20;
    println!("a is {}", a);
    let mut sum = 0;
    for i in 0..10 {
        sum += i;
    }
    println!("sum is {}", sum);

    let a: u8 = 0b1111;
    let b: u8 = 0b0011;
    println!("a & b is {:04b}", a & b);
    println!("a | b is {:04b}", a | b);

    let a: u8 = 0x02;
    println!("a << 1 is {}", a << 1);
    println!("a >> 1 is {}", a >> 1);

    let a = true;
    let b = false;
    println!("a && b is {}", a && b);
    println!("a || b is {}", a || b);

    let a = true;
    let b = !a;
    println!("a is {}, b is {}", a, b);
}
