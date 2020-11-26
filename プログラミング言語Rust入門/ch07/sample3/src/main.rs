fn main() {
    let v = vec![1, 2, 3, 4, 5];
    print!("for is ");
    for i in &v {
        print!("{} ", i);
    }
    println!("");
    print!("for and iter is ");
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");

    let v = vec![1, 2, 3, 4, 5];
    let mut p = v.iter();
    println!("p is {:?}", p);
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());

    let v = vec![1, 2, 3, 4, 5];
    println!("by loop");
    let mut p = v.iter();
    loop {
        let x = p.next();
        if x == None {
            break;
        }
        println!("x is {}", x.unwrap());
    }
    println!("by while");
    let mut p = v.iter();
    while let Some(x) = p.next() {
        println!("x is {}", x);
    }

    let v = vec![1, 2, 3, 4, 5];
    let lst = v.iter().map(|x| x * 10);
    for i in lst {
        println!("i is {}", i);
    }
}
