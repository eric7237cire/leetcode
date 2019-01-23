use rand::distributions::{Distribution, Uniform};
use rand::prelude::StdRng;
use rand::SeedableRng;
use crate::util::grid::Grid;
use rulinalg::matrix::Matrix;
use nalgebra::*;

pub fn solve_all_cases()
{
    let mut rng: StdRng = SeedableRng::seed_from_u64(42);
    let is_connected = Uniform::from(0..2i16);

    let num_vertices = 4;
    //let matrix = vec![ vec![0; num_vertices]; num_vertices];
    let mut matrix : Grid<f64> = Grid::new(num_vertices, num_vertices);

    for i in 0..num_vertices {
        for j in i+1..num_vertices {
            let ic = is_connected.sample(&mut rng) as f64;
            matrix[ (i,j) ] = -ic ;
            matrix[ (j,i) ] = -ic;

            matrix[ (i,i) ] += ic;
            matrix[ (j,j) ] += ic;
        }
    }        

     let dm = DMatrix::from_row_slice(num_vertices, num_vertices, &matrix.data);

    let dm = DMatrix::from_row_slice(4, 4, &[
   2.0, 0f64, -1.0, -1.0,
        0., 2., -1.0, -1.0,
        -1.0, -1.0, 3., -1.0,
        -1.0, -1.0, -1.0, 3.
]);
    //let dm = Matrix::new(num_vertices, num_vertices, matrix.data.clone());

    println!("{:#.6?}",matrix);

    for row in 0..num_vertices {
        println!("{:?}", dm.row(row).iter().collect::<Vec<_>>());
        // println!("{:?}", dm);
    }

    println!("{}", dm.slice( (1,1), (num_vertices-1, num_vertices-1)).determinant());
    
}