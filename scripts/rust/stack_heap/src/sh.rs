#![allow(dead_code)]
use std::mem; // to find out how much bytes is occupied

struct Point
{
  x: f64,
  y: f64
}

fn origin() -> Point // marking the origin
{
  Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
  let p1 = origin(); // let p1:Point - stack allocated
  let p2 = Box::new(origin()); // heap allocated - the pointer to this location is stored in the stack

  println!("p1 takes up {} bytes", mem::size_of_val(&p1));
  println!("p2 takes up {} bytes", mem::size_of_val(&p2));


  // unboxing it
  let p3 = *p2; // to get the boxed value to stack - assign the address value to p3
  println!("{}", p3.x);
  /*
  OUTPUT: 
    
    p1 takes up 16 bytes
    p2 takes up 8 bytes
    0

    p1 takes up 16 bytes.
    size of f64 = 8 bytes, hence f64*2

    p2 takes up only 8 bytes - a pointer to a Point - store the address
    - the address takes 64 buts or 8 bytes- bcoz mac is 64 bit machine.
  */
}