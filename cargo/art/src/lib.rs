/// # Art
///
/// A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match c1 {
            PrimaryColor::Red => {
                match c2 {
                    PrimaryColor::Yellow => SecondaryColor::Orange,
                    PrimaryColor::Blue => SecondaryColor::Purple,
                    _ => SecondaryColor::Green,
                }
            },
            PrimaryColor::Yellow => {
                match c2 {
                    PrimaryColor::Red => SecondaryColor::Orange,
                    PrimaryColor::Blue => SecondaryColor::Green,
                    _ => SecondaryColor::Purple,
                }
            },
            PrimaryColor::Blue => {
                match c2 {
                    PrimaryColor::Red => SecondaryColor::Purple,
                    PrimaryColor::Yellow => SecondaryColor::Green,
                    _ => SecondaryColor::Orange,
                }
            },
        }
    }
}
