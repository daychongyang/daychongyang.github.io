fn main() {
  let v = vec![1, 2, 3];

  let third = &v[2];

  println!("The third element is {}", third);

  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  }
}
