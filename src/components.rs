use bevy::prelude::Component;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn size_square_is_created_sized_square() {
        let expected = Size {
            width: 3.14,
            height: 3.14,
        };
        let actual = Size::square(3.14);

        assert_eq!(actual, expected);
    }
}
