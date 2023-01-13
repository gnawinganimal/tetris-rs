
#[derive(PartialEq, Eq, Debug)]
pub struct Shape {
    points: Vec<(isize, isize)>,
}

impl Shape {
    pub fn area(&self) -> usize {
        self.points.len()
    }
 
    pub fn dsp(self, d: (isize, isize)) -> Self {
        Self {
            points: self.points.iter().map(|p| (p.0 + d.0, p.1 + d.1)).collect(),
        }
    }
}

impl From<&[(isize, isize)]> for Shape {
    fn from(slice: &[(isize, isize)]) -> Self {
        Self {
            points: Vec::from(slice),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displace_shape() {
        let a = Shape::from([(2, 3), (2, 4), (2, 5), (3, 5)].as_slice()).dsp((3, -2));

        assert_eq!(a, Shape::from([(5, 1), (5, 2), (5, 3), (6, 3)].as_slice()));
    }
}
