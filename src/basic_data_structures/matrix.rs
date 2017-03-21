#![allow(dead_code)]

extern crate num;
extern crate std;

pub struct Matrix<T> {
    data: Vec<Vec<T>>
}

impl<T> Matrix<T>
where T: num::Zero + std::clone::Clone + num::One
{
    fn new(height: usize, width: usize) -> Option<Self> {
        if (height == 0) || (width == 0) {
            None
        }
        else
        {
            Some(Matrix::<T>{ data: vec![vec![T::zero(); width]; height]})
        }
    }

    fn new_identity_matrix(matrix_size: usize) -> Option<Self> {
        if matrix_size == 0 {
            return None
        }
        let mut id_matrix = Matrix::<T>{ data: vec![vec![T::zero(); matrix_size]; matrix_size]};
        for i in 0..matrix_size {
            id_matrix.data[i][i] = T::one();
        }
        Some(id_matrix)
    }

    fn transpose(&self) -> Matrix<T> {
        let tran_height = self.data[0].len();
        let tran_width = self.data.len();
        let mut transposed_matrix = match Matrix::<T>::new(tran_height, tran_width) {
            Some(tm) => tm,
            None => panic!("Couldn't transpose given matrix, wrong sizes")
        };


        transposed_matrix
    }

}

#[cfg(test)]
mod test
{
    use basic_data_structures::matrix;

    #[test]
    fn identity_matrix() {
        match matrix::Matrix::<i32>::new_identity_matrix(3) {
            Some(m) => assert_eq!(m.data, vec![vec![1, 0, 0],vec![0, 1, 0],vec![0, 0, 1]]),
            None => panic!()
        }
    }

    #[test]
    fn matrix_of_zero_width() {
        match matrix::Matrix::<i32>::new(5, 0) {
            Some(_) => panic!(),
            None => ()
        }
    }

    #[test]
    fn matrix_of_zero_height() {
        match matrix::Matrix::<i32>::new(0, 5) {
            Some(_) => panic!(),
            None => ()
        }
    }

    #[test]
    fn identity_matrix_of_zero_size() {
        match matrix::Matrix::<i32>::new_identity_matrix(0) {
            Some(_) => panic!(),
            None => ()
        }
    }
}
