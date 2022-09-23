fn main() {
    let  x= 5;  // initial value assigned to x is 5

    let x = x +1;  //we are reusing the variable name. The memory address is different for this value

    let x = x * 2; //yet another value of the same variable name. Even the type can change in shadowing

    println!("The value of x is {}", x);

    /* 
    Perfect example of a case where shadowing is necessary:
    let spaces = "    ";
    let spaces = spaces.len(); 
    data type has changed in this case
    */
}
