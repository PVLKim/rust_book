use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // type annotation for input parameter and return value (optional)
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly..");
        // thread::sleep(Duration::from_secs(2));
        num
    };

    // first call to this closure infers type of input value as String and hence, later call with int will result in error
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // this will produce type error

    // borrow immutably
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list); // saved but not evaluated yet (lazy)
    println!("Before calling closure: {:?}", list);
    only_borrows(); // can be called as a function
    println!("After calling closure: {:?}", list);
    println!("");

    // borrow mutably
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // println!("Before calling closure: {:?}", list); //not possible to do immutable borrow here, since mut borrow happened earlier
    borrows_mutably(); 
    println!("After calling closure: {:?}", list);
    println!("");

    // takes ownership (moves it to a new thread)
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || {
        println!("From thread: {:?}", list)
    }).join().unwrap();

    // FnMut
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // attempt to count number of calls to the closure (4)
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1; // increments counter
        r.width // return an orderable value
    });
    println!(
        "{:#?}, sorted in {num_sort_operations} operations",
        list
    );

}
