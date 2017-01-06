extern crate vmar;
extern crate time;
extern crate gnuplot;
extern crate rand;

use rand::distributions::{IndependentSample, Range};

use vmar::algorithms::k_means;
use vmar::basic_data_structures::point;
use gnuplot::{Figure, Color, AxesCommon};

fn main() {
    let mut rng = rand::thread_rng();
    let random_range = Range::new(-5.,100.);
    let mut data: Vec<point::Point<f64>> = vec!();
    for _ in 0..100
    {
        data.push(point::Point::new(random_range.ind_sample(&mut rng) as f64 / 3., random_range.ind_sample(&mut rng) as f64 / 3.));
    }
    for _ in 0..100
    {
        data.push(point::Point::new(30. + random_range.ind_sample(&mut rng) as f64 / 3.,60. + random_range.ind_sample(&mut rng) as f64 / 3.));
    }
    for _ in 0..100
    {
        data.push(point::Point::new(60. + random_range.ind_sample(&mut rng) as f64 / 3., random_range.ind_sample(&mut rng) as f64 / 3.));
    }
    let mut algo = k_means::KMeans::new_with_points_and_region_count(data, 3);
    algo.region_centers = vec!(point::Point::new(0., 0.), point::Point::new(20., 20.),point::Point::new(50., 7.));
    algo.calculate_centers(1000);
    let results = match algo.show_data_for_centers() {
        Ok(data_result) => data_result,
        Err(error_message) => panic!(error_message)
    };
    let colors = [Color("black"), Color("red"), Color("green")];
    let mut fg = Figure::new();
    for i in 0..3
    {
        fg.axes2d()
        .set_x_range(gnuplot::AutoOption::Fix(-5.), gnuplot::AutoOption::Fix(100.))
        .set_y_range(gnuplot::AutoOption::Fix(-5.), gnuplot::AutoOption::Fix(100.))
        .points(&[results[i].0.x], &[results[i].0.y], &[colors[i], gnuplot::PlotOption::PointSymbol('+'), gnuplot::PlotOption::PointSize(5.)]);
        let mut x: Vec<f64> = vec![];
        let mut y: Vec<f64> = vec![];
        for p in &results[i].1
        {
            x.push(p.x);
            y.push(p.y);
        }
        fg.axes2d()
        .set_x_range(gnuplot::AutoOption::Fix(-5.), gnuplot::AutoOption::Fix(100.))
        .set_y_range(gnuplot::AutoOption::Fix(-5.), gnuplot::AutoOption::Fix(100.))
        .points(x, y, &[colors[i], gnuplot::PlotOption::PointSymbol('*'), gnuplot::PlotOption::PointSize(1.)]);
    }
    fg.show();
}
