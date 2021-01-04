struct Foo {}

impl Foo {}

// The newtype
pub struct Bar(Foo);


impl Bar {
    pub fn new() -> Bar {}
}

fn main() {
    let b = Bar::new();


    // Fpp and Bar are type incompatible, the following do not type check.
    // let f: Foo = b;
    // let b: Bar = Foo {};
}