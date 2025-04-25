use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub byte: usize,
    pub bit: u8,
}
impl fmt::Display for Point {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "byte: {}, bit: {}", self.byte, self.bit)
    }
}

impl Point {
    #[inline]
    pub fn new(row: usize, col: usize, n: usize) -> Self {
        Self {
            byte: row * n / 8 + col / 8,
            bit: 0b0000_0001 << 8 - col % 8 - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_row() {
        let p: Point = Point::new(0, 8, 16);
        assert_eq!(p.byte, 1);
        assert_eq!(p.bit, 0b1000_0000);
    }

    #[test]
    fn third_row() {
        let p: Point = Point::new(2, 8, 16);
        assert_eq!(p.byte, 5);
        assert_eq!(p.bit, 0b1000_0000);
    }

    #[test]
    fn second_col() {
        let p: Point = Point::new(2, 1, 16);
        assert_eq!(p.byte, 4);
        assert_eq!(p.bit, 0b0100_0000);
    }

    #[test]
    fn eightth_col() {
        let p: Point = Point::new(2, 7, 16);
        assert_eq!(p.byte, 4);
        assert_eq!(p.bit, 0b0000_0001);
    }

    #[test]
    fn thirteenth_col() {
        let p: Point = Point::new(2, 13, 16);
        assert_eq!(p.byte, 5);
        assert_eq!(p.bit, 0b0000_0100);
    }
}
