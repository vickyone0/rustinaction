fn main() {
  let x = 5.to_string();
  drop(x);
  println!("{}", x); // This line would cause a compile-time error
}