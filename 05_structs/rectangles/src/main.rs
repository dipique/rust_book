fn main() {
    let r = Rectangle {
        width: 30,
        height: 50
    };

    println!("Area is: {}", area_struct(&r)); // immutable reference
    println!("Rectangle value: {:?}", r);
    println!("Or, here's a prettier display: {:#?}", r);

    // use area function in Rectangle
    println!("Area is: {}", r.area());

    // Rust has automatic referencing and de-referencing, and thus
    // r.area() is the same as (&r).area() are the same. So even
    // though .area() takes the parameter &self, you're not required
    // to create the reference to self.

    let r2 = Rectangle {
        width: 10,
        height: 15
    };

    let r3 = Rectangle {
        width: 31,
        height: 15
    };

    let r4 = Rectangle {
        width: 60,
        height: 65
    };

    println!("r can hold r2: {}", r.can_hold(&r2));
    println!("r can hold r3: {}", r.can_hold(&r3));
    println!("r can hold r4: {}", r.can_hold(&r4));

    // "associated functions" are the equivalent of c#
    // static methods and do not have a self parameter
    
    // they are accessed with the :: operator instead
    // of the dot operator
    println!("Creating square of size 11: {:?}", Rectangle::square(11));
}

#[derive(Debug)] // use this straight to add debugging ({:?}) functionality
struct Rectangle {
    width: u32,
    height: u32
}

// there can be multiple impl blocks for the same type,
// but it's not necessary here
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn area_struct(r: &Rectangle) -> u32 {
    r.height & r.width
}

/////////////////////

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

/////////////////////

fn separate() { 
    let w: u32 = 44;
    let l: u32 = 30;
    println!("The area is {} pixels squared", area_separate(w, l));
}

fn area_separate(w: u32, l: u32) -> u32 {
    w * l
}
