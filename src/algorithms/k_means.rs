extern crate std;
extern crate num;

use basic_data_structures::point;
use std::clone::Clone;


#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct KMeans<T>
{
    pub data_points: Vec<point::Point<T>>,
    pub number_of_regions: u32,
    pub region_centers: Vec<point::Point<T>>
}

impl<T> KMeans<T>
where T: std::clone::Clone + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Div<Output = T>
+ Copy + Default + num::ToPrimitive + num::FromPrimitive + num::Zero + PartialEq
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        let km = KMeans::<T>{data_points: vec!(), number_of_regions: 0, region_centers: vec!()};
        km
    }

    #[allow(dead_code)]
    pub fn new_with_points_and_region_count(data:Vec<point::Point<T>>, regions_count: u32) -> Self {
        let km = KMeans::<T>{data_points: data, number_of_regions: regions_count, region_centers: vec!()};
        km
    }

    #[allow(dead_code)]
    pub fn calculate_centers(&mut self, iterations: u64) -> Option<String>{
        for _ in 0..iterations
        {
            let mut points_and_centers: Vec<Vec<point::Point<T>>> = vec![vec![]; self.number_of_regions as usize];
            for point in self.data_points.clone()
            {
                match self.nearest_center(point) {
                    Ok(index) => points_and_centers[index].push(point),
                    Err(message) => return Some(message)
                }
            }
            self.region_centers = self.recalculate_center(&points_and_centers);
        }
        None
    }

    fn nearest_center(&self, p: point::Point<T>) -> Result<usize, String>
    {
        if self.number_of_regions < 2 {
            if self.number_of_regions < 1 {
                return Err("No enough regions to calculate".to_string())
            }
            else {
                return Ok(0)
            }
        };
        let mut nearest_index: usize = 0;
        let mut nearest_distance = p.distance(&self.region_centers[0]);
        for i in 1..self.number_of_regions
        {
            if p.distance(&self.region_centers[i as usize]) < nearest_distance
            {
                nearest_distance = p.distance(&self.region_centers[i as usize]);
                nearest_index = i as usize;
            }
        }
        Ok(nearest_index)
    }

    pub fn recalculate_center(&self, points_near_centers: &Vec<Vec<point::Point<T>>>) -> Vec<point::Point<T>>
    {
        let mut new_centers: Vec<point::Point<T>> = vec!();
        for points in points_near_centers
        {
            let new_point = KMeans::calculate_center_of_points(&points);
            if new_point.is_some()
            {
                new_centers.push(new_point.unwrap());
            }
            else
            {
                let l = new_centers.len();
                new_centers.push(point::Point::copy_point(&self.region_centers[l]));
            }
        }
        new_centers
    }

    fn calculate_center_of_points(points: &Vec<point::Point<T>>) -> Option<point::Point<T>>
    {
        let point_count = T::from_usize(points.len()).unwrap();
        if point_count == T::zero()
        {
            return None
        }
        let mut center_point = point::Point::new(T::zero(), T::zero());
        for p in points
        {
            center_point.move_point(p.x,p.y);
        }
        let center_point = point::Point::new(center_point.x / point_count, center_point.y / point_count);
        Some(center_point)
    }

    #[allow(dead_code)]
    pub fn show_data_for_centers(&self) -> Result<Vec<(point::Point<T>,Vec<point::Point<T>>)>, String>
    {
        let mut sliced_data_set: Vec<(point::Point<T>,Vec<point::Point<T>>)> = vec!();
        for center in &self.region_centers
        {
            let tmp_vec: Vec<point::Point<T>> = vec!();
            sliced_data_set.push((*center, tmp_vec));
        }
        for p in &self.data_points
        {
            match self.nearest_center(*p) {
                Ok(index) => sliced_data_set[index].1.push(*p),
                Err(message) => return Err(message)
            }
        }
        Ok(sliced_data_set)
    }

}

#[cfg(test)]
mod test
{
    use basic_data_structures::point;
    use algorithms::k_means;
    #[test]
    fn algorithm_validity() {
        let mut data: Vec<point::Point<f64>> = vec!();
        for _ in 0..10
        {
            data.push(point::Point::new(3.,2.));
        }
        for _ in 0..10
        {
            data.push(point::Point::new(10.,17.));
        }
        let mut algo = k_means::KMeans::new_with_points_and_region_count(data, 2);
        algo.region_centers = vec!(point::Point::new(0., 0.), point::Point::new(20., 20.));
        algo.calculate_centers(5);
        assert_eq!(algo.region_centers[0], point::Point::new(3., 2.));
        assert_eq!(algo.region_centers[1], point::Point::new(10., 17.));
    }

    #[test]
    fn data_for_centers_result_test() {
        let mut data: Vec<point::Point<f64>> = vec!();
        for _ in 0..10
        {
            data.push(point::Point::new(3.,2.));
        }
        for _ in 0..10
        {
            data.push(point::Point::new(10.,17.));
        }
        let mut algo = k_means::KMeans::new_with_points_and_region_count(data, 2);
        algo.region_centers = vec!(point::Point::new(0., 0.), point::Point::new(20., 20.));
        algo.calculate_centers(5);
        let mut first_data_set: Vec<point::Point<f64>> = vec!();
        let mut second_data_set: Vec<point::Point<f64>> = vec!();
        for _ in 0..10
        {
            first_data_set.push(point::Point::new(3.,2.));
        }
        for _ in 0..10
        {
            second_data_set.push(point::Point::new(10.,17.));
        }
        let result_data: Vec<(point::Point<f64>, Vec<point::Point<f64>>)> = vec!(
            (point::Point::new(3.,2.),first_data_set),
            (point::Point::new(10.,17.),second_data_set));
        match algo.show_data_for_centers() {
            Ok(shown_data) => assert_eq!(result_data, shown_data),
            Err(_) => panic!()
        }
    }

    #[test]
    fn no_region_count_set_test() {
        let mut km = k_means::KMeans::new();
        let mut data: Vec<point::Point<f64>> = vec!();
        for _ in 0..10
        {
            data.push(point::Point::new(3.,2.));
        }
        for _ in 0..10
        {
            data.push(point::Point::new(10.,17.));
        }
        km.data_points = data;
        match km.calculate_centers(5) {
            Some(error_message) => assert_eq!(error_message, "No enough regions to calculate".to_string()),
            None => panic!("No error message")
        }
    }
}
