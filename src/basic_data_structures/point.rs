extern crate std;
extern crate num;

#[derive(Debug, Copy, Clone)]
pub struct Point <T>{
    pub x: T,
    pub y: T
}

impl<T> Point<T>
where T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + num::ToPrimitive + num::FromPrimitive + num::Zero
{
    #[allow(dead_code)]
    pub fn distance(&self, p: &Point<T>) -> f64 {
        (((p.x - self.x)*(p.x - self.x) + (p.y-self.y)*(p.y-self.y))).to_f64().unwrap().sqrt()
    }

    #[allow(dead_code)]
    pub fn new(pos_x: T, pos_y: T) -> Self {
        Point::<T>{x: pos_x, y: pos_y}
    }

    pub fn copy_point(poi: &Point<T>) -> Self {
        Point::new(poi.x,poi.y)
    }

    pub fn move_point(&mut self, x_axis: T, y_axis: T)
    {
        self.x = self.x + x_axis;
        self.y = self.y + y_axis;
    }
}

impl<T> PartialEq for Point<T>
    where T: std::clone::Clone + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + PartialEq
    {
    fn eq(&self, other: &Point<T>) -> bool {
        if self.x == other.x {
            if self.y == other.y {
                return true
            }
        }
        false
    }
}

#[cfg(test)]
mod test{
    use basic_data_structures::point;
    ///Points
    #[test]
    fn point_creation() {
        assert_eq!(point::Point {x: 5., y: 10.}, point::Point::new(5.,10.));
    }

    #[test]
    fn point_distance() {
        assert_eq!(point::Point::new(5.,10.).distance(&point::Point::new(50.,10.)),45.);
        assert_eq!(point::Point::new(5.,10.).distance(&point::Point::new(5.,10.)),0.);
    }
}
