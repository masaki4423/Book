fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new();
    let mut v = Vec::<i32>::new();

    let mut v: Vec::<&str> = Vec::new();
    let mut v: Vec::<&String> = Vec::new();
    let mut v: Vec::<f64> = Vec::new();

    let v = [10, 20, 30, 40, 50];
    print_i32(&v);
    let v = ['A', 'B', 'C', 'D', 'E'];
    print_char(&v);
    let v = ["one", "two", "three", "four", "five"];
    print_str(&v);

    let v = [10, 20, 30, 40, 50];
    print(&v);
    let v = ['A', 'B', 'C', 'D', 'E'];
    print(&v);
    let v = ["one", "two", "three", "four", "five"];
    print(&v);
}

fn print_i32(a: &[i32]) {
    print!("a is ");
    for i in a {
        print!("{} ", i);
    }
    println!("");
}

fn print_char(a: &[char]) {
    print!("a is ");
    for i in a {
        print!("{} ", i);
    }
    println!("");
}

fn print_str(a: &[&str]) {
    print!("a is ");
    for i in a {
        print!("{} ", i);
    }
    println!("");
}

fn print<T>(a: &[T]) where T: std::fmt::Debug {
    print!("a is ");
    for i in a {
        print!("{:?} ", i);
    }
    println!("");
}