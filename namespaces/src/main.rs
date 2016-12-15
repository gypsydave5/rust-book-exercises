pub mod a {
    pub mod series {
        pub mod of {
            pub fn namespaces() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use a::series::of;
use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    of::namespaces();
}
