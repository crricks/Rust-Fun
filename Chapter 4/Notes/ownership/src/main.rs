fn main() {
    // variable refers to literal string - hardcoded value valid until end of scope
    let s = "hello";

    // strings not literals stored on heap
    // :: operator that allows us to namespace from function under String type
    // valid until out of scope
    let mut s = String::from("hello"); // can be mutated

    s.push_str(", world"); // appends a literal to a String

    println!("{}", s);

    // do not copy data, copy directions to location on heap, size, and capacity
    let s1 = String::from("hello");
    let s2 = s1; // s1 then becomes invalid - s1 moved into s2

    // deep copy using clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // copy used only on stack-only data
    let x = 5;
    let y = x; // x is still usable

    // once function is called using String, no longer valid
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s); 
    					// s no longer valid




}
