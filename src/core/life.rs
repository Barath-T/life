use super::point::Point;
use std::fmt; 

#[derive(Debug)]
pub struct Life{
    plane: Vec<u8>,
    // n should be multiple of 8
    n: usize
}

impl fmt::Display for Life {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.plane.len() {
            if i % (self.n / 8) == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{:08b} ", self.plane[i])?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl Life {
    pub fn new(n: usize) -> Self{
        let mut plane: Vec<u8> = Vec::with_capacity(n * n / 8);

        for _ in 0..(n * n / 8) {
            plane.push(0b0000_0000);
        }

        Self{
            plane,
            n
        }
    }

    pub fn live(&mut self) {
        let mut population: Vec<Vec<u8>> = vec![vec![0; self.n]; self.n];
        let directions: [(isize, isize); 8] = [(0,1), (1,0), (1, 1), (-1, 0), (0, -1), (-1, -1), (1, -1), (-1, 1)];
        for i in 0..self.n {
            for j in 0..self.n {
                for direction in directions {
                    let next: (isize, isize) = (i as isize+direction.0, j as isize +direction.1);
                    if  next.0>= 0 &&  next.1>= 0 && next.0 < self.n as isize && next.1 < self.n as isize{
                        let point = Point::new(next.0 as usize, next.1 as usize, self.n);
                        if self.plane[point.byte] & point.bit != 0 {
                            if population[i][j] < 4 {
                                population[i][j] += 1;
                            }
                        }
                    }
                }

            }
        }

        for i in 0..self.n {
            for j in 0..self.n {
                if population[i][j] < 2 || population[i][j] > 3{
                    let point = Point::new(i, j, self.n);
                    self.plane[point.byte] &= !point.bit;
                }
                else if population[i][j] == 3{
                    let point = Point::new(i, j, self.n);
                    self.plane[point.byte] |= point.bit;
                }
            }
        }
    }

    pub fn apply(&mut self, high_points: Vec<Point>){
        for p in high_points {
            self.plane[p.byte] |= p.bit;
        }
    }
}


