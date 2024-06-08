use std::fmt::Display;

#[derive(Debug)]
struct ImportantText<'a> {
    text: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This compiles even with no lifetimes given even tho references are used for the parameters and
// return object
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

impl<'a> ImportantText<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantText<'a> {
    // This function looks look this in terms of lifetimes
    // fn announce_and_return_part<'a, 'b>(&'a self, announcement:: &'b str) -> &'a str // <- this
    // part is usually not infered but since one of the parameters is self the return lifetime type
    // will be the same as self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.text
    }
}

// putting it all together
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
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

fn main() {
    {
        let string1 = String::from("abcd");
        let string2 = String::from("Hello there!");
        let result = longest(&string1, &string2);
        println!("string2: {result}");
    }
    {
        let novel = String::from("Call me Smart. Many moons ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantText {
            text: first_sentence,
        };
        println!("i: {:?}", i);
        // Lifetime Elision up next
    }
    // The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.
    // The 'static lifetime tells the compiler that the affected reference can live for the entire
    // duration of the program
    // Before specifying 'static as a lifetime for a reference think about whether the reference you have actually lives the entire lifetime of your program or not,
    // and whether you want it to. Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a
    // mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the 'static lifetime.
    {
        let s: &'static str = "I have a static lifetime.";
    }
}
