/**
 * This code is the Julia equivalent of 
 * f(x::String) = println("Got a string")
 * f(x::Number) = println("Got a number")
 * the magic happens at the impls and also the `into` and `from` implementation
 * pretty straight forward tbh
 */

pub enum Types<'a> {
    Int(i32),
    Str(&'a str),
    Strn(String),
}

impl<'a> From<i32> for Types<'a> {
    fn from(v: i32) -> Self {
        Self::Int(v)
    }
}

impl<'a> From<&'a str> for Types<'a> {
    fn from(v: &'a str) -> Self {
        Self::Str(v)
    }
}

impl<'a> From<String> for Types<'a> {
    fn from(v: String) -> Self {
        Self::Strn(v.to_string())
    }
}

pub fn f<'a>(x: impl Into<Types<'a>>) {
    match x.into() {
        Types::Int(_int) => println!("got a number"),
        Types::Str(_str) => println!("Got a string"),
        Types::Strn(_str) => println!("Got an *actual* string  lol"),
    }
}
