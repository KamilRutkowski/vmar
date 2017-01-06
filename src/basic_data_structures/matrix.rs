extern crate num;
extern crate std;

pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where T: num::Zero + std::clone::Clone + num::One
{
    #[allow(dead_code)]
    fn new(height: usize, width: usize) -> Self {
        Matrix::<T>{ data: vec![vec![T::zero(); width]; height]}
    }

    #[allow(dead_code)]
    fn new_identity_matrix(matrix_size: usize) -> Self {
        let mut id_matrix = Matrix::<T>{ data: vec![vec![T::zero(); matrix_size]; matrix_size]};
        for i in 0..matrix_size {
            id_matrix.data[i][i] = T::one();
        }
        id_matrix
    }
}

#[cfg(test)]
mod test
{
    use basic_data_structures::matrix;
    #[test]
    fn identity_matrix() {
        assert_eq!(matrix::Matrix::<i32>::new_identity_matrix(3).data, vec![vec![1, 0, 0],vec![0, 1, 0],vec![0, 0, 1]]);
    }
}
