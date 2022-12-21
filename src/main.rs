use std::fmt::Display;

fn main() {
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    {
        // This does not work, as the RUST compiler realizes that string2 does not live as long as string1.
        // Lifetimes always take the smallest lifetime for two identical arguments.

        // As humans, we can look at this code and see that string1 is longer than string2 
        // and therefore result will contain a reference to string1. 
        // Because string1 has not gone out of scope yet, a reference to string1 will still be valid 
        // for the println! statement. However, the compiler can’t see that the reference is valid 
        // in this case. We’ve told Rust that the lifetime of the reference returned by the longest function 
        // is the same as the smaller of the lifetimes of the references passed in. 
        // Therefore, the borrow checker disallows the code in Listing 10-23 as possibly having an invalid reference.
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// is a LIFETIME. It is the implementation of the abstract
// concept of "scope or a lifetime". It is used
// to denote the relationship of lifetimes between the arguments
// and the return type. In this example, it denotes that the returned reference,
// will be valid as long as both the parameters are valid.
// The lifetime annotationsgo in the function signature & are part of the contract of the function,
// much like the types in the signature.
// The generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y
fn longest<'a>(x: &'a str, y: &'a str)->  &'a str {
    let s: &'static str = "I have a static lifetime."; 
    println!("I am static {}", s);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
//  The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.
//  The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
//  If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, 
//  the compiler will stop with an error.
//  These rules apply to fn definitions as well as impl blocks.

//  1) first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. 
//  In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); 
//  a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

//  2) The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
//   fn foo<'a>(x: &'a i32) -> &'a i32.

//  3) The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, 
//  the lifetime of self is assigned to all output lifetime parameters. 
//  This third rule makes methods much nicer to read and write because fewer symbols are necessary.

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}