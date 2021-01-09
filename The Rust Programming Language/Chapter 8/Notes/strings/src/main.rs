fn main() {
    // rust has only string slice str in core language
    // String literal stored in program's binary and are therefore str's

    // create new empty String
    let mut s = String::new();

    // create new String with data
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string(); // also works directly

    let s = String::from("initial contents"); // equivalent to 13

    // appending to String
    let mut s = String::from("foo");
    s.push_str("bar"); // uses str because don't necessarily want to take ownership

    // adding single character
    let mut s = String::from("lo");
    s.push('l');

    // combine existing strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s3 takes ownership of s1 and appends copy of s2 - returns ownership of result
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    // concatenate multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // easier to read and doesn't take ownership

    // cannot index Strings because stored in bytes
    // use chars method if need to iterate
    for c in "hello".chars() {
    	println!("{}", c);
    }

    // can also use bytes to access bytes
    for b in "hello".bytes() {
    	println!("{}", b);
    }


}
