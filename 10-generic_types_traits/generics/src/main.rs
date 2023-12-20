use std::cmp::PartialOrd;

// Monomophization will turn each generic type into concrete type at compile time
fn largest<T: PartialOrd>(list: &[T]) -> &T { // restricts to types that implement PartialOrd trait
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<X1,Y1> {
    x: X1,
    y: Y1,
}
impl<X1,Y1> Point<X1,Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }

    // signature and return type don't have to be the same
    fn mixup<X2,Y2>(self, other: Point<X2,Y2>) -> Point<X1,Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
impl Point<f32,f32> { // the following method only works for Point<f32>
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}



enum Option<T> {
    Some(T),
    None,
}

fn main() {

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

}