#[derive(Debug)] // works if we want to avoid implementing Debug trait
struct Rectangle { // allows more readable parameter space
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle { // multiple impl work as well
    // associated function, that doesn't need an instance of itself as a param
    fn square(size: u32) -> Self { 
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // implemented as method
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) // ref is needed to leave `rect1` in main's scope
    );
    println!("rect1 is {:#?}", rect1); // {:?} will display in one line
    
    // Associated function called with ::
    let sq = Rectangle::square(3);
    println!("sq is {:?}", sq);

    // 
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height 
}