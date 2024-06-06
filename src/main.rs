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
}
