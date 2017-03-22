//! # Art
//!
//! A library for modeling artistic concepts.
//!
//! pub use

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) => Some(SecondaryColor::Orange),
            (PrimaryColor::Yellow, PrimaryColor::Red) => Some(SecondaryColor::Orange),
            (PrimaryColor::Red, PrimaryColor::Blue) => Some(SecondaryColor::Purple),
            (PrimaryColor::Blue, PrimaryColor::Red) => Some(SecondaryColor::Purple),
            (PrimaryColor::Blue, PrimaryColor::Yellow) => Some(SecondaryColor::Green),
            (PrimaryColor::Yellow, PrimaryColor::Blue) => Some(SecondaryColor::Green),
            _ => None
        }
    }
}

