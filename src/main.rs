fn main() {
   let sum = 5 + 10;
   
   let difference = 95.5 - 4.3;

   let product = 4 * 30;

   let quotient = 56.7 / 32.2;

   let remainder = 43 % 5;

   println!("sum: {} difference: {} product: {} quotient: {} remainder: {}", sum, difference, product, quotient, remainder);

   let t = true;
   let f: bool = false;

   println!("the bolean is: {} and explicitly declared boolean is: {}",t, f);

   let c = 'z';
   let z = 'Z';
   let heart_eyed_cat = '😻';

   println!("characters are declared with a single quote in Rust. c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);
}
