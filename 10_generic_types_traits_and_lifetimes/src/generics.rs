pub fn get_largest_ex() {
    let num_list = vec![10, 30, 25, 100, 67];
    let largest = largest(&num_list);
    println!("The largest number is {}", largest);
}

fn largest<T: PartialOrd + Copy>(num_list: &[T]) -> T {
    let mut largest = num_list[0];
    for &num in num_list {
        if num > largest {
            largest = num
        }
    }
    largest
}

// we could avoid using the copy trait by instead using the Clone
// trait (which would be slower). We could also return &T which
// wouldn't require either trait.

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // sort of re-implements property getting, but not quite since
    // it is a function and returns a reference
    fn x(&self) -> &T {
        &self.x
    }
}

// we can also create a function that applies only to one variant
impl Point<i32> {
    fn say_waddup(&self) {
        println!("I AM AN INTEGER! {}", &self.y);
    }
}

fn use_point() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 0.6, y: 10.4 };
    integer.say_waddup();
    // float.say_waddup(); // will not compile
}

struct MGPoint<T,U> {
    x: T,
    y: U,
}

impl<T,U> MGPoint<T,U> {
    fn mixup<V, W>(self, other: MGPoint<V,W>) -> MGPoint<T,W> {
        MGPoint {
            x: self.x,
            y: other.y
        }
    }
}

fn use_multi_generic_point() {
    // let point = Point { x: 5, y: 0.2 }; won't compile because x and y have to be same type
    let point = MGPoint { x: 5, y: 0.2 };
}

// this is how options/results are implemented
enum MyOption<T> {
    Some(T),
    None
}
enum MyResult<T,U> {
    Some(T),
    Err(U),
}