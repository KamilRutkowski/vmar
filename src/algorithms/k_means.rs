#![allow(dead_code)]

extern crate std;
extern crate num;

use std::clone::Clone;

pub enum DataState<T>
where T: PointProperties<T> + std::default::Default
{
    Ok(KMeans<T>),
    NoPoints,
    NoCenters,
    MoreCentersThanPoints,
    NotEqualDimensions
}

pub enum KMResult<T>
where T: PointProperties<T> + Default
{
    NotCalculated,
    Calculated(Vec<(T,Vec<T>)>)
}

#[derive(Debug, Clone)]
enum KMeansResult<T>
where T: PointProperties<T> + std::default::Default
{
    NotYetCalculated,
    AlgorithmResult(KMeansResultStruct<T>)
}

#[derive(Debug, Clone)]
struct KMeansResultStruct<T>
where T: PointProperties<T> + std::default::Default
{
    result: Vec<(T,Vec<T>)>
}

#[derive(Debug, Clone)]
pub struct KMeans<T>
where T: PointProperties<T> + std::default::Default
{
    data_points: Vec<T>,
    region_centers: Vec<T>,
    algorithm_result: KMeansResult<T>
}

pub trait PointProperties<T>
where T: PointProperties<T> + std::default::Default
{
    fn distance(&self, second_point: &T) -> f64;
    fn dimensionality(&self) -> u64;
    fn calculate_new_center_for_given_points(&mut self, points: &Vec<T>);
}

impl<T> KMeans<T>
where T: PointProperties<T> + std::clone::Clone + Copy + Default//+ std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Div<Output = T>
//+ Copy + Default + num::ToPrimitive + num::FromPrimitive + num::Zero + PartialEq
{
    pub fn new(data: Vec<T>, start_centers: Vec<T>) -> DataState<T> {
        match KMeans::check_if_correct_input_data(&data, &start_centers) {
            Some(err) => return err,
            None => {
                let km = KMeans::<T>{data_points: data, region_centers: start_centers, algorithm_result: KMeansResult::NotYetCalculated};
                DataState::Ok(km)
            }
        }
    }

    fn check_if_correct_input_data(data: &Vec<T>, start_centers: &Vec<T>) -> Option<DataState<T>>
    {
        if data.len() == 0 {
            return Some(DataState::NoPoints);
        }
        if start_centers.len() == 0 {
            return Some(DataState::NoCenters);
        }
        if data.len() < start_centers.len() {
            return Some(DataState::MoreCentersThanPoints);
        }
        let data_dimension = data[0].dimensionality();
        for p in data {
            if p.dimensionality() != data_dimension {
                return Some(DataState::NotEqualDimensions);
            }
        }
        for p in start_centers {
            if p.dimensionality() != data_dimension {
                return Some(DataState::NotEqualDimensions);
            }
        }
        None
    }

    pub fn change_data_points(&mut self, data: Vec<T>) -> Option<DataState<T>>
    {
        match KMeans::check_if_correct_input_data(&data, &self.region_centers) {
            Some(err) => return Some(err),
            None => {
                self.data_points = data;
                self.algorithm_result = KMeansResult::NotYetCalculated;
                None
            }
        }
    }

    pub fn change_center_points(&mut self, centers: Vec<T>) -> Option<DataState<T>>
    {
        match KMeans::check_if_correct_input_data(&self.data_points, &centers) {
            Some(err) => return Some(err),
            None => {
                self.region_centers = centers;
                self.algorithm_result = KMeansResult::NotYetCalculated;
                None
            }
        }
    }

    pub fn change_data_and_center_points(&mut self, data: Vec<T>, centers: Vec<T>) -> Option<DataState<T>>
    {
        match KMeans::check_if_correct_input_data(&self.data_points, &centers) {
            Some(err) => return Some(err),
            None => {
                self.data_points = data;
                self.region_centers = centers;
                self.algorithm_result = KMeansResult::NotYetCalculated;
                None
            }
        }
    }

    pub fn find_possible_centers(&mut self, iterations: u64) -> Option<String>{
        for _ in 0..iterations
        {
            let mut points_near_center_at_index: Vec<Vec<T>> = vec![vec![]; self.region_centers.len()];
            for point in self.data_points.clone()
            {
                match self.find_nearest_center_index(&point) {
                    Ok(index) => points_near_center_at_index[index].push(point),
                    Err(message) => return Some(message)
                }
            }
            self.recalculate_center_points(&points_near_center_at_index);
            let mut sorted_centers_and_data: Vec<(T,Vec<T>)> = vec![];
            for center_index in 0..self.region_centers.len()
            {
                sorted_centers_and_data.push(
                    (self.region_centers[center_index].clone(),
                     points_near_center_at_index[center_index].clone())
                );
            }
            self.algorithm_result = KMeansResult::AlgorithmResult(
                KMeansResultStruct{
                    result: sorted_centers_and_data
                });
        }
        None
    }

    fn find_nearest_center_index(&self, p: &T) -> Result<usize, String>
    {
        match self.region_centers.len() {
            0 => return Err("No enough regions to calculate".to_string()),
            1 => return Ok(0),
            _ => {
                let mut nearest_center_index: usize = 0;
                let mut nearest_center_distance = p.distance(&self.region_centers[0]);
                for i in 1..self.region_centers.len()
                {
                    if p.distance(&self.region_centers[i as usize]) < nearest_center_distance
                    {
                        nearest_center_distance = p.distance(&self.region_centers[i as usize]);
                        nearest_center_index = i as usize;
                    }
                }
                Ok(nearest_center_index)
            }
        }
    }

    fn recalculate_center_points(&mut self, points_near_centers: &Vec<Vec<T>>)
    {
        for center_index_to_recalculate in 0..points_near_centers.len()
        {
            self.region_centers[center_index_to_recalculate]
            .calculate_new_center_for_given_points(&points_near_centers[center_index_to_recalculate]);
        }
    }

    pub fn show_centers_with_data(&self) -> KMResult<T>
    {
        match self.algorithm_result.clone() {
            KMeansResult::NotYetCalculated    => KMResult::NotCalculated,
            KMeansResult::AlgorithmResult(r)  => KMResult::Calculated(r.result)
        }
    }
}


#[cfg(test)]
mod test
{
    use basic_data_structures::point2d;
    use algorithms::k_means;

    #[test]
    fn algorithm_validity() {
        let mut data: Vec<point2d::Point2d> = vec!();
        for _ in 0..10
        {
            data.push(point2d::Point2d::new(3.,2.));
        }
        for _ in 0..10
        {
            data.push(point2d::Point2d::new(10.,17.));
        }
        let centers = vec!(point2d::Point2d::new(0., 0.), point2d::Point2d::new(20., 20.));
        let mut algo: k_means::KMeans<point2d::Point2d>;
        match k_means::KMeans::new(data, centers) {
            k_means::DataState::Ok(val) => algo = val,
            k_means::DataState::NoPoints | k_means::DataState::NoCenters | k_means::DataState::MoreCentersThanPoints |
            k_means::DataState::NotEqualDimensions => panic!("KMeans was not created")
        };
        match algo.find_possible_centers(5) {
            Some(err) => panic!(err),
            None => ()
        };
        match algo.show_centers_with_data() {
            k_means::KMResult::NotCalculated                 => panic!("K means is not calculated"),
            k_means::KMResult::Calculated(centers_and_data)  => {
                assert_eq!(centers_and_data[0].0, point2d::Point2d::new(3., 2.));
                assert_eq!(centers_and_data[1].0, point2d::Point2d::new(10., 17.));
            }
        }
    }
    #[test]
    #[should_panic(expected = "No data points supplied")]
    fn no_data_points_supplied() {
        let data: Vec<point2d::Point2d> = vec!();
        let centers = vec!(point2d::Point2d::new(0., 0.), point2d::Point2d::new(20., 20.));
        match k_means::KMeans::new(data, centers) {
            k_means::DataState::Ok(_) => panic!("Expected NoPoints panic, got Ok"),
            k_means::DataState::NoPoints => panic!("No data points supplied"),
            k_means::DataState::NoCenters | k_means::DataState::MoreCentersThanPoints |
            k_means::DataState::NotEqualDimensions => panic!("Expected NoPoints panic, got other")
        };
    }

    #[test]
    #[should_panic(expected = "No center points supplied")]
    fn no_center_points_supplied() {
        let mut data: Vec<point2d::Point2d> = vec!();
        for _ in 0..10
        {
            data.push(point2d::Point2d::new(3.,2.));
        }
        for _ in 0..10
        {
            data.push(point2d::Point2d::new(10.,17.));
        }
        let centers: Vec<point2d::Point2d> = vec!();
        match k_means::KMeans::new(data, centers) {
            k_means::DataState::Ok(_) => panic!("Expected NoCenters panic, got Ok"),
            k_means::DataState::NoCenters => panic!("No center points supplied"),
            k_means::DataState::NoPoints | k_means::DataState::MoreCentersThanPoints |
            k_means::DataState::NotEqualDimensions => panic!("Expected NoCenters panic, got other")
        };
    }

    #[test]
    #[should_panic(expected = "More centers than points supplied")]
    fn more_centers_than_points_supplied() {
        let mut data: Vec<point2d::Point2d> = vec!();
        for _ in 0..2
        {
            data.push(point2d::Point2d::new(3.,2.));
        }
        let centers = vec!(point2d::Point2d::new(0., 0.), point2d::Point2d::new(20., 20.), point2d::Point2d::new(30., 30.));
        match k_means::KMeans::new(data, centers) {
            k_means::DataState::Ok(_) => panic!("Expected MoreCentersThanPoints panic, got Ok"),
            k_means::DataState::MoreCentersThanPoints => panic!("More centers than points supplied"),
            k_means::DataState::NoPoints | k_means::DataState::NoCenters |
            k_means::DataState::NotEqualDimensions => panic!("Expected MoreCentersThanPoints panic, got other")
        };
    }

}
