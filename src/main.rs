fn main() {
  let arr = [10, 20, 30, 40, 50];

  for it in arr.iter().rev() {
    println!("The value is:{}", it);
  }
}
