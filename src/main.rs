fn main() {
  //Arrays in rust must have a fixed length, once declared, they cannot be modified either way

  // they are useful when one wants the data to be allocated to a stack rather than heap
  //also used to ensure that one always has a fixed number of elements
  // the alternative where one can grow or shrink the size is to use VECTORS

  let a = [1,2,3,4,5];
  let index_one = a[1];

  println!("The length of the array is: {}", a.len());
  println!("The value at index one in the array is: {} ", index_one);
}
