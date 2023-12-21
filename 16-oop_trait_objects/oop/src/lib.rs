// This demonstrates encapsulation
// To the user we only expose public methods such as add and remove element from list
// By calling those public methods, the private `update_average` method automatically
// recalculates average 
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) { // private method
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

}


// This demonstrates polymorphism (code that works with multiple)
// To enable that, you need to use trait object <Box<dyn Trait>>
// trait object points to both an instance of a type implementing our trait
// and a table used to look up trait methods on that type in runtime!
pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // this enables vector to contain any object that
                                        // impl trait Draw (could be several different structs)
}
impl Screen{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

// If you only have homogeneous collections(list of components contains only 1 type)
// then using generics with trait bounds is preferrable(better perf)
pub struct ScreenT<T: Draw> {
    pub components: Vec<T>, // this forces to use any, but single type in the collection
}
impl<T> ScreenT<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {}
}