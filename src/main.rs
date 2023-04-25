fn main() {
    println!("Hello, world!");

    /* Ownership in Variables */

    /* Simple types */
    let x: i32 = 5;
    let mut y: i32 = x;
    /* here rust copies the value of variable x into y.
    So this copy will occur for all the simple type that are stored on the stack */
    y += 1;
    println!("{},{}", x, y);

    /* Reference/Complex types */
    let s1: String = String::from("hello world");
    let s2: String = s1;
    /* Incase of above as it is a reference type,
    we will expect that it will perform
    shallow copy - a new reference is created for same memory where value is stored
    But incase of rust, to ensure the memory safety it moves the value from s1 to s2
    and invalidates s1. So if we try to access s1 after this line it throws error saying
    that we are trying to borrow after the move.*/
    // println!(s1);
    println!("Moved value: {}", s2);

    /* So to perform copy we can use clone function to do so */
    let s3: String = s2.clone();
    println!("Cloned values: {}, {}", s2, s3);

    /* Ownership in functions */
    let x = 5;
    makes_copy(5); // same as variable, rust copies the value of simple types
    println!("After function param copy: {}", x);

    let some_str = String::from("hello func");
    takes_ownership(some_str);
    /* Similar to assigning a variable,
    once we had passed a variable to a function as argument
    the value will be moved to that function (owner of that value will be that var function)
    So, above we had passed some_str to the function and now the value is moved to some_string param.
    So again if we try to print some_str after above line, we will get same error
    saying that we are trying to borrow the moved value
    */
    // println!("Trying to print borrowd val {}", some_str)

    /* Similar to passing variable to func as above
    if we return some value from the function the ownership of value is moved from the func to stored var */
    let s1: String = gives_ownership();
    println!("Gets ownership: {}", s1);

    /* References and borrowings */
    /* Rules of references
       1. At any given time, we can have either one mutable reference
       or any number of immutable references. (Not mix of both)
       2. References must always be valid and point to valid data.
    */
    /* Lets see how we can reuse the value after passing to a func call */
    /* 1. We can pass and return back the passed val along with return value */
    let s6: String = String::from("hello");
    let (s6, len) = calculate_length_takes_and_gives_ownership(s6);
    println!("Takes and gives ownership: The length of string {s6} is {len}");

    /* 2. We can get rid of above things of returning back passed value by using references.
        So, instead of passing a value to the param, we can pass reference.
    */
    let s7: String = String::from("hello");
    let len = calculate_length(&s7);
    println!("References: The length of string {s7} is {len}");
    /* Here this function just gets the reference of s7
    Something like s-> s7->val
    So even after s is dropped, the value sustains as s7 is still pointing to it */

    /* Now, let say if we wanted to update the passed string when we pass the reference of it */
    /* By default, the data that refers cannot be borrowed as mutable without taking ownership */
    /* To do so, we need to pass mutable reference to mutable variable */
    let mut s7 = String::from("hello");
    change(&mut s7);
    println!("References: Changed value of s7 {s7}");

    /* Mutable references -
    We can have only one mutable reference at a particular point
    for a particular piece of data in a particular scope */

    let mut s8 = String::from("hello");
    let r1: &mut String = &mut s8;
    /* Below is not a valid one as r1
    is already a mut ref to the s8 in this scope*/
    // let r2: &mut String = &mut s8;
    // println!("Mutable ref r2 of s8 after r1 scope{}", r2);

    /* Instead of mutable we can have multiple immutable references but not mix of both until within a scope*/
    println!("Mutable ref of s8 r1: {}", r1);

    /* And we also should note that the scope of variable ends after its last usage.
    So we can use after it */
    let r3: &mut String = &mut s8;
    println!("Mutable ref r3 of s8 after r1 scope: {}", r3);

    /* Dangling references - A reference that points to a invalid data */
    // let ref_to_nothing = dangle();
    /* Rust prevents this to happen to make sure that we are not doing any memory unsafe operations */

    /* Slices
    -  Slices will let us reference contiguous sequence
    of elements within a collection instead of referencing the entire collection
    - Just like references slices will not take ownership of underlying data.
    - slices are represented by type &str
    - Slices are actually string literals,that means these are stored directly in binary.
    and string slices actually references to that location in binary
    */
    let s: String = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
    /* By above slices, hello & s will point to 0 index but with different lengths
        i.e., hello -> 0 index with length 5
              world -> 6 index with length 5
        &s[..5] -> from starting of string to 4th index
        &s[6..] -> from 6th index to end of the string
        &s[..] -> entire string
    */
    let fir_word = first_word(&s);
    // s.clear();
    /* Here s.clear() will raises an error as it mutates the string and
    above we had used a immutable reference to the same string which is not possible*/
    println!("Slices: First word of {s} is {fir_word}");

    /* We can use slices on different types of collections */
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    println!("Slices: array {:?}", slice);
}

fn takes_ownership(some_string: String) {
    println!("In function takes_ownership: {}", some_string);
    /* Here some_string is printed and after this func block
    this variable is destroyed so after this function we cant access it  */
}

fn makes_copy(some_integer: i32) {
    println!("In function makes_copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello gives");

    some_string
}

fn calculate_length_takes_and_gives_ownership(s: String) -> (String, usize) {
    let length: usize = s.len();
    (s, length)
}

fn calculate_length(s: &String) -> usize {
    /* Here this function just gets the reference of passed string */
    /* Something like s-> passed_string->val */
    /* So even after s is dropped, the value sustains as passed_string is still pointing to it */
    s.len()
}

fn change(s: &mut String) {
    /* Here the s is a mutable reference to the passed value */
    /* Any changes to this will effect the value of passed string */
    s.push_str(", rust world");
}

/* fn dangle() -> &String {
    let s: String = String::from("hello");

    /* this function's return type contains
    a borrowed value of s, but there is no value for it to be borrowed after this scope  */
    &s
} */

fn first_word(s: &str) -> &str {
    /* Here param s is of type string literal which
    can accept both string references and string literals/slices */
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}
