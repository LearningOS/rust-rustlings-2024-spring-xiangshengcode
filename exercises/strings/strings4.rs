// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");                                  // &str
    string("red".to_string());                            // String
    string(String::from("hi"));                           // String
    string("rust is fun!".to_owned());                    // String
    string_slice("nice weather".into());                  // &str (`.into()` is smart enough to infer as &str)
    string(format!("Interpolation {}", "Station"));        // String
    string_slice(&String::from("abc")[0..1]);             // &str (slice of a String)
    string_slice("  hello there ".trim());                // &str (.trim() returns &str directly)
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // String
    string_slice(&"mY sHiFt KeY iS sTiCkY".to_lowercase()); // &str
}

