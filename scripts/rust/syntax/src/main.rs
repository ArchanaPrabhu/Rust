#[derive(Debug)]
pub struct Bed{
    size:i32,
    count:u32,
}

#[derive(Debug)]
pub enum Room {
    Kitchen(i32),
    Bedroom(Bed),
    Lounge(i32, String),
}

fn main() {
    // println!("Hello, world!"); 
    // loop_to_5();
    // vect_print();
    // strings();

    use self::Room::*;
    let t = Bedroom(Bed{size:50, count:2});
    println!("Hello from the {:?}", t);

    /* 
    match t {
        Room::Kitchen(n) => println!("The room is a kitched with {} rooms ", n),
        d => println!("{:?}", d),
    }

    if let Lounge(n, s) = t {
        println!("Its a {} lounge with {} cupboards", s, n);
    } 
    */

    let v = match t {
        Kitchen(n) => n,
        d=> 0,
    };

    println!("v = {:?}", v);
}

fn loop_to_5() {
    for n in 0..10 {
        println!("{} ", n);
    }
}

fn vect_print() {
    let v = vec![1, 2, 3, 4];
    for i in v {
        println!("{}", i);
    }
}

fn strings() {
    let s =String :: from("Hello 文字"); // we are using string since "Hello" is an &str
    // &str is a pointer but String :: from is equivalent to a vector

    println!("S Len = {}", s.len()); // returns the number of bytes

      
    for c in s.chars() {
        println!("{} ", c);
    }

    for c in s.bytes() {
        println!("{}", c);
    }

    for (i, c) in s.chars().enumerate() { // to find the character position of the string
        println!("{} = {}", i, c);
    }

    for (i, c) in s.char_indices() {
        println!("{} = {}", i, c);
    }
    println!("Num of l's {} ", count_l(&s));
}

fn count_l(s:&str)->i32{
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res += 1;
        }
    }
    res
}




