fn main() {
    let v = vec![10, 20, 30, 40, 50];
    print!("v is ");
    for i in &v {
        print!("{} ", i);
        // let x: i32 = i;
        let x: i32 = *i;
    }
    println!("");

    let v = vec![10, 20, 30, 40, 50];
    print!("v is ");
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");

    let v = vec![10, 20, 30, 40, 50];
    print!("v is ");
    for (i, x) in v.iter().enumerate() {
        print!("{}:{} ", i, x);
    }
    println!("");

    print!("FOR is ");
    for i in 0..10 {
        print!("{} ", i);
    }
    println!("");

    print!("FOR is ");
    for i in 0..10 {
        if i == 5 {
            break;
        }
        print!("{} ", i);
    }
    println!("");

    print!("FOR is ");
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!("");

    print!("WHILE is ");
    let mut i = 0;
    while i < 10 {
        print!("{} ", i);
        i += 2;
    }
    println!("");

    print!("LOOP is ");
    let mut i = 0;
    loop {
        if i >= 10 {
            break;
        }
        print!("{} ", i);
        i += 1;
    }
    println!("");
}
