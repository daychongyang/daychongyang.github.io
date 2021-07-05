trait Summary {
  fn summarize(&self) -> String;
}

struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {}({})", self.headline, self.author, self.location)
  }
}

fn main() {
  let article = NewsArticle {
    headline: String::from("123"),
    location: String::from("234"),
    author: String::from("567"),
    content: String::from("897"),
  };

  println!("1 new article: {}", article.summarize());
}
