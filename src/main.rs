fn largest_i32(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn largest_char(list: &[char]) -> &char {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn largest_t<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

struct Point<T, U> {
  x: T,
  y: U,
}

struct Point2<T> {
  x: T,
  y: T,
}

impl<T> Point2<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point2<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

struct Point3<X1, Y1>{
  x: X1,
  y: Y1,
}

impl<X1, Y1> Point3<X1, Y1>{
  fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2>{
    Point3 {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  // removing duplication by extrating function
  // generics help to dry duplicates

  let number_list = vec![11, 2, 31, 4, 57];
  let mut largest = &number_list[0];

  for number in &number_list {
    if number > largest {
      largest = number;
    }
  }

  println!("the largest number is {}", largest);

  let another_number_list = vec![100, 4, 74, 99, 456, 1, 0];
  let mut another_largest = &another_number_list[0];

  for num in &another_number_list {
    if num > another_largest {
      another_largest = num;
    }
  }

  println!("the largest number is {}", another_largest);

  let ultimate_number_list = vec![132, 625, 101, 569, 471];
  let result = largest_i32(&ultimate_number_list);
  println!("largest number in this list is: {}", result);

  println!();
  let char_list = vec!['a', 'b', 'c', 'd', 'e', 'f'];
  let bigger_char = largest_char(&char_list);
  println!("biggest char in this list is: {}", bigger_char);

  println!();
  let float_list = vec![1.7, 2.2, 3.7, 4.2, 5.2, 6.2];
  let biggest_float = largest_t(&float_list);
  println!("biggest float in this list is: {}", biggest_float);

  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 2.0 };

  println!();
  //let wont_work = Point { x: 0, y: 0.0 };
  // ^^^ expected integer, found floating-point number

  let both_integer = Point { x: 1, y: 2 };
  let both_float = Point { x: 1.0, y: 2.0 };
  let integer_float = Point { x: 1, y: 2.0 };

  println!();
  let p = Point2 { x: 0, y: 1 };
  println!("p.x = {}", p.x());

  println!();
  let p1 = Point3 { x: 0, y: 1.4 };
  let p2 = Point3 { x: "hello", y: 'c' };
  let p3 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
  println!();


}