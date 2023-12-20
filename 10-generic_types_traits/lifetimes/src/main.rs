// Lifetimes ensure that references are valid as long as we need them to be
// Most of the time they are implicit and inferred, but we must annotate
// lifetimes when the lifetimes of refs could be related in a few different ways.

// Main aim of lifetimes is to prevent dangling references
// Lifetime annotations don't change how long any of the refs live
// rather, they describe rel-ships of the lifetimes of multiple 
// references to each other. 

// The following tells that for some lifetime 'a, func's params are string slices
// that live at least as long as lifetime 'a. In practice, it means that the lifetime
// of the reference returned by the longet func is the same as the smaller of 
// the lifetimes of the values referred to by the func arguments
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Here there's no lifetime for y, since it has no rel-ship with return value
fn first<'a>(x: &'a str, y: &str) -> &'a str { x }

// We can define structs to hold references, but we would need to add lifetimes
// this annotation means an instance of ImportantExcerpt can't outlive the ref
// it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime elision rules allow programmer to not specify lifetime annotations
// for some cases. For that the compiler tries to apply 3 rules, and if after doing
// so, there's still uncertainty about some lifetimes, it stop with an error:
// Rule 1. Assign a lifetime parameter to each param that's a reference 
//    => fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
// Rule 2. If there's exactly one input lifetime param, that lieftime is assigned 
//         to all output lifetime params => fn foo<'a>(x: &'a i32) -> &'a i32
// Rule 3. If there're multiple input lifetime params, but one of them is &self
//         or &mut self, the lifetime of self gets assigned to all output lifetime params

// Static lifetime denotes that the affected ref can live for the entire duration of program


// Final example of func using generics, trait bound and lifetimes
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {result}"); // won't compile

    // 
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
