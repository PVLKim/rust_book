fn main() {
    // To leave s1 in scope after using it in a function, we borrow reference to it with &s1
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // ! To change s in a function, it needs to be mutable and mutable reference should be borrowed
    // ! only 1 mutable reference is allowed in the same scope 
      // (unless first reference is used, before the scope for second one starts)
    // ! similarly, a mutable reference is only allowed if there's no "active" immutable 
      // reference in scope (multiple immutable references are ok)
    
    let mut s = String::from("hello");
    change(&mut s);
    println!("The mutated s is : {}", s);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
