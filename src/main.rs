mod multiple_dispatch; // not actually multiple dispatch :) something like it
use crate::multiple_dispatch:: {
    f,
    Types,
};
fn main() {
    f(Types::from("joe"));
    f(Types::from(String::from("joe")));
}