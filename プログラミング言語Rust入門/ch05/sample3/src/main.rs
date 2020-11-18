fn main() {
    no_param();

    one_param(10);
    two_param(10, 20);

    let ret = two_param_and_return(10, 20);
    println!("ret is {}", ret);

    str_param("rust");

    let mut s = String::new();
    str_param_complex(&mut s);
    println!("s is {}", s);

    let ret = str_param_and_return("rust");
    println!("ret is {}", ret);

    let v = vec![1, 2, 3, 4, 5];
    let sum = vec_param(&v);
    println!("sum is {}", sum);

    let v = vec_return(10);
    for i in v {
        print!("{} ", i);
    }
    println!("");

    let mut v = vec![1, 2, 3, 4, 5];
    vec_change(&mut v);
    for i in v {
        print!("{} ", i);
    }
    println!("");
}

fn no_param() {
    println!("called no_param");
}

fn one_param(x: i32) {
    println!("called one_param, x is {}", x);
}

fn two_param(x: i32, y: i32) {
    println!("called two_param, {} and {}", x, y);
}

fn two_param_and_return(x: i32, y: i32) -> i32 {
    println!("called two_param_and_return, {} and {}", x, y);
    x + y
}

fn str_param(s: &str) {
    println!("called str_param, s is {}", s);
}

fn str_param_complex(s: &mut String) {
    *s = String::from("hello");
}

fn str_param_and_return(s: &str) -> String {
    println!("called str_param_and_return, s is {}", s);
    let ret = format!("hello {} world.", s);
    ret
}

fn vec_param(v: &Vec<i32>) -> i32 {
    println!("called vec_param");
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

fn vec_return(max: i32) -> Vec<i32> {
    println!("called vec_return");
    let mut v = Vec::new();
    for i in 0..max {
        v.push(i);
    }
    v
}

fn vec_change(v: &mut Vec<i32>) {
    println!("called vec_change");
    for i in v {
        *i = *i * 10;
    }
}