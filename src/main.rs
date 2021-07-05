#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let rect = Rectangle {
    width: 30,
    height: 50,
  };

  let other = Rectangle::square(29);

  println!("The rect is {:#?}", rect);

  println!("The area of the rectangle is {} square pixels", rect.area());

  println!("Can react hold other? {}", rect.can_hold(other));
}
