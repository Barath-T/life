use crate::point::Point;

pub fn generate_move(row: usize, col: usize, n: usize) -> Vec<Point> {
    let mut high_points: Vec<Point> = Vec::with_capacity(5);
    high_points.push(Point::new(row + 0, col + 1, n));
    high_points.push(Point::new(row + 1, col + 2, n));
    high_points.push(Point::new(row + 2, col + 0, n));
    high_points.push(Point::new(row + 2, col + 1, n));
    high_points.push(Point::new(row + 2, col + 2, n));

    high_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let points = generate_move(1, 1, 16);
        let mut _points: Vec<Point> = Vec::with_capacity(5);
        _points.push(Point::new(1, 2, 16));
        _points.push(Point::new(2, 3, 16));
        _points.push(Point::new(3, 1, 16));
        _points.push(Point::new(3, 2, 16));
        _points.push(Point::new(3, 3, 16));
        for i in 0..points.len(){
            assert_eq!(points[i], _points[i]);
        }
        
    }
}
