fn main() {
   println!("Welcome to the functions example!");
  /*
  Functions, just like variables in Rust, they are declared using the camel case
  Rust doesn't care whether the function is declared before or after the main function
  May or may not have parameters/arguments
  In function signatures, all the parameters must have their types declared

  Statements do not return any values
  Expressions do return values e.g x+1 is an expression to do some addition
  */
  let z = five();
  let x = plus_one(5);
  println!("The value of x is: {}",x);
}

fn five ()-> i32 { //this function is supposed to return a value of type i32
   5
}

fn plus_one(x: i32) -> i32 { //this function is supposed to return a value of type i32 and it has a parameter of type i32 as well
  x+1
}
