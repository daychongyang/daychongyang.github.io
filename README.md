# Daily

## 变量与不可变性

1. 变量默认是不可改变的(`immutable`)
2. 可在变量名之前加`mut`来使其可变

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}
```

### 变量与常量的区别

1. 常量总是不能变
2. 声明常量使用 `const` 关键字声明, 且必须注明值的类型
3. 常量可以在任何作用域中声明
4. 常量只能被设置为常量表达式么不能是函数调用结果或任何其他只能在运行时计算出的值

```rust
const MAX_POINTS:u32 = 100_000;
```

### 隐藏

定义一个与之前变量同名的新变量, 而新变量会隐藏之前的变量

```rust
fn main () {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is {}", x);
}
```

## 数据类型

- 标量
  - 整型
  - 浮点型
  - 字符类型
  - 布尔类型
- 复合
  - 元组
  - 数组

## 函数

语句: 执行一些操作但不返回值的指令
表达式: 计算并产生一个值

```rust
fn main() {
  let x = 5;

  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of x is {}", x);
  println!("The value of y is {}", y);
}
```

## 注释

### 单行注释

```rust
// hello, world!
```

### 文档注释

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
```

## 控制流

### if 表达式

```rust
fn main() {
  let number = 7;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }

  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {}", number);
}
```

### 循环

#### `loop`

```rust
fn main() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The value of result is {}", result);
}
```

#### `while`

```rust
fn main() {
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("LIFTOFF!!!");
}
```

### 遍历集合

```rust
fn main() {
  let arr = [10, 20, 30, 40, 50];

  let mut index = 0;

  while index < 5 {
    println!("The value is: {}", arr[index]);

    index += 1;
  }
}
```

```rust
fn main() {
  let arr = [10, 20, 30, 40, 50];

  for it in arr.iter() {
    println!("The value is:{}", it);
  }
}
```

## 所有权

> 所有权(系统)是 Rust 最为与众不同的特性, 它让 Rust 无需垃圾回收即可保障内存安全.

### 所有权规则

1. Rust 中的每一个值都有一个被称为其"所有者"(_owner_)的变量
2. 值在任一时刻有且只有一个所有者
3. 当所有者(变量)离开作用域(一个项在程序中有效的范围), 这个值将被丢弃

### 变量与数据交互的方式
#### 移动

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);

// error[E0382]: use of moved value: `s1`
//  --> src/main.rs:5:28
//   |
// 3 |     let s2 = s1;
//   |         -- value moved here
// 4 |
// 5 |     println!("{}, world!", s1);
//   |                            ^^ value used here after move
//   |
//   = note: move occurs because `s1` has type `std::string::String`, which does
//   not implement the `Copy` trait
```

#### 克隆

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

#### `栈` 拷贝 

```rust
let x = 5;
let y = x;

println("x = {}, y = {}", x, y);
```


### 所有权与函数

1. 将值传递给函数在语义上与给变量赋值相似, 向函数传递值可能会移动或者复制, 就像赋值语句一样
2. 返回值可以转移所有权


```rust
fn main() {
  let s = String::from("hello"); // s 进入作用域

  take_ownership(s); // s 的值移动到函数里...
                    // ... s 不再有效

  let x = 5; // x 进入作用域

  make_copy(x); // x 应该移动到函数里, 但 i32 是 `Copy` 的, 所以后面可以继续使用 x
                // x 仍然有效
} // x 先移出了作用域, 然后是 s, 但因为 s 的值已经被移走, 所以不会有特殊操作

fn take_ownership(some_string: String) {
  println!("{}", some_string);
} // 这里, some_string 移除作用域并调用 `drop` 方法, 占用内存被释放 

fn make_copy(some_integer: i32) {
  println!("{}", some_integer);
} // 这里, some_integer 移出作用域, 不会有特殊操作
```


```rust
fn main() {
  let s1 = gives_ownership(); // gives_ownership 将返回值移给 s1

  let s2 = String::from("hello"); // s2 进入作用域

  let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中, 并将其返回值移给 s3
} // 这里, s3 移出作用域并被丢弃. s2 也移出作用域, 但已被移走所有什么也不会发生, s1 移出作用域并被丢弃

fn gives_ownership() -> String { // gives_ownership 将返回值移动给调用它的函数
  let some_string = String::from("hello"); // some_string 进入作用域

  some_string // 返回 some_string 并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String { // takes_and_gives_back 将返回值移动给调用它的函数,a_string 进入作用域
  a_string // 返回 a_string 并移出给调用的函数
}
```