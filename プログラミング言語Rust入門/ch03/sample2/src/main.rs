fn main() {
    let x = 100;
    let y = x;
    println!("x is {}", x);
    println!("y is {}", y);

    // move
    // let x = String::from("Hello");
    // let y = x;
    // println!("x is {}", x);
    // println!("y is {}", y);

    // borrow
    let x = String::from("Hello");
    let len = string_length(&x);
    println!("len is {}", len);
    println!("x is {}", x);

    // bind
    let mut x = 100;
    println!("x is {}", x);
    x = 200;
    println!("x is {}", x);

    // shadowing
    let x = 100;
    println!("x is {}", x);
    let x = 200;
    println!("x is {}", x);

    let x = 100;
    let x = "masuda";

    // scope
    let x = 100;
    println!("x is {}", x);
    {
        let x = 200;
        println!("x is {}", x);
    }
    println!("x is {}", x);

    let ans = add_two(10, 20);
    println!("ans is {}", ans);
    let ans = add_one(30);
    println!("ans is {}", ans);

    let a = Sample::new(10);
    let ans = a.inc();
    println!("ans is {}", ans);
    let ans = a.add(20);
    println!("ans is {}", ans);

}

fn string_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn test(x: i32) -> i32 {
    // let mut ans = x;
    // if x < 0 {
    //     ans = 0;
    // }
    // if x > 100 {
    //     ans = 100;
    // }
    // ans

    // if x < 0 {
    //     0
    // } else if x > 100 {
    //     100
    // } else {
    //     x
    // }

    let ans = if x < 0 {
        0
    } else if x > 100 {
        100
    } else {
        x
    };
    ans
}

fn add_two(x: i32, y: i32) -> i32 {
    x + y
}

fn add_one(x: i32) -> i32 {
    x + 1
}

struct Sample {
    x: i32,
}
impl Sample {
    fn new(x: i32) -> Sample {
        Sample {x: x}
    }
    fn inc(&self) -> i32 {
        self.x + 1
    }
    fn add(&self, x: i32) -> i32 {
        self.x + x
    }
}