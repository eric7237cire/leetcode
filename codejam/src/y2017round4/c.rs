use crate::util::grid::Grid;
use nalgebra::*;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::StdRng;
use rand::SeedableRng;
use rulinalg::matrix::Matrix;

const MAX_VERTEX: usize = 22;
struct Spanning
{
    is_connected: [[bool; MAX_VERTEX]; MAX_VERTEX],
    matrix: [[f64; MAX_VERTEX]; MAX_VERTEX],
}

impl Spanning
{
    fn new() -> Self
    {
        Spanning {
            is_connected: [[false; MAX_VERTEX]; MAX_VERTEX],
            matrix: [[0f64; MAX_VERTEX]; MAX_VERTEX],
        }
    }

    fn count(&mut self, num_vertices: usize) -> usize
    {
        for i in 0..num_vertices {
            for j in 0..=i {
                self.matrix[i][j] = 0.0;
                self.matrix[j][i] = 0.0;
;
            }
        }

        for i in 0..num_vertices {
            for j in 0..i {
                let ic = if self.is_connected[i][j] { 1f64 } else { 0f64 };

                self.matrix[i][j] = -ic;
                self.matrix[j][i] = -ic;

                self.matrix[i][i] += ic;
                self.matrix[j][j] += ic;
            }
        }

        //gaussian elimination
        let mut n = num_vertices;
        n -= 1;
        for e in 0..n {
            //assert(fabs(b[e][e]) > eps);
            for i in e + 1..n {
                let coeff = -self.matrix[i][e] / self.matrix[e][e];
                for j in e..n {
                    self.matrix[i][j] += coeff * self.matrix[e][j];
                }
            }
        }

        let mut ans = 1.0;
        for i in 0..n {
            ans *= self.matrix[i][i];
        }

        (ans + 0.5) as usize
    }
}

pub fn solve_all_cases()
{
    let mut rng: StdRng = SeedableRng::seed_from_u64(42);
    let is_connected = Uniform::from(0..2i16);

    let num_vertices = 4;
    let mut vec2d = vec![vec![0f64; num_vertices]; num_vertices];
    let mut matrix: Grid<f64> = Grid::new(num_vertices, num_vertices);

    for i in 0..num_vertices {
        for j in i + 1..num_vertices {
            let ic = is_connected.sample(&mut rng) as f64;
            matrix[(i, j)] = -ic;
            matrix[(j, i)] = -ic;

            matrix[(i, i)] += ic;
            matrix[(j, j)] += ic;

            vec2d[i][j] = -ic;
            vec2d[j][i] = -ic;

            vec2d[i][i] += ic;
            vec2d[j][j] += ic;
        }
    }

    let mut g: Grid<f64> = Grid::new(num_vertices, num_vertices);
    for i in 0..num_vertices {
        for j in 0..num_vertices {
            g[(i, j)] = vec2d[i][j];
        }
    }
    println!("{:#.6?}", g);

    //gaussian elimination
    let mut n = num_vertices;
    n -= 1;
    for e in 0..n {
        //assert(fabs(b[e][e]) > eps);
        for i in e + 1..n {
            let coeff = -vec2d[i][e] / vec2d[e][e];
            for j in e..n {
                vec2d[i][j] += coeff * vec2d[e][j];
            }
        }
    }

    let mut g: Grid<f64> = Grid::new(num_vertices, num_vertices);
    for i in 0..num_vertices {
        for j in 0..num_vertices {
            g[(i, j)] = vec2d[i][j];
        }
    }

    let mut ans = 1.0;
    for i in 0..n {
        ans *= vec2d[i][i];
    }

    let dm = DMatrix::from_row_slice(num_vertices, num_vertices, &matrix.data);

    /*let dm = DMatrix::from_row_slice(4, 4, &[
       2.0, 0f64, -1.0, -1.0,
            0., 2., -1.0, -1.0,
            -1.0, -1.0, 3., -1.0,
            -1.0, -1.0, -1.0, 3.
    ]);*/
    //let dm = Matrix::new(num_vertices, num_vertices, matrix.data.clone());

    println!("{:#.6?}", matrix);

    println!("{:#.6?}\n{}", g, ans);

    for row in 0..num_vertices {
        println!("{:?}", dm.row(row).iter().collect::<Vec<_>>());
        // println!("{:?}", dm);
    }

    println!(
        "{}",
        dm.slice((1, 1), (num_vertices - 1, num_vertices - 1))
            .determinant()
    );
}

#[cfg(test)]
mod test_2017_round4_c
{
    use super::*;

    #[test]
    fn test_spanning_tree_count()
    {
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);
        let is_connected = Uniform::from(0..2i16);

        let num_vertices_gen = Uniform::from(4..12usize);

        let mut spanning = Spanning::new();

        'test_loop: for _ in 0..100 {
            let num_vertices = num_vertices_gen.sample(&mut rng);

            let mut matrix = vec![vec![0f64; num_vertices]; num_vertices];
            // let mut matrix: Grid<f64> = Grid::new(num_vertices, num_vertices);

            for i in 0..num_vertices {
                for j in 0..i {
                    let ic = is_connected.sample(&mut rng) as f64;
                    matrix[i][j] = -ic;
                    matrix[j][i] = -ic;

                    matrix[i][i] += ic;
                    matrix[j][j] += ic;

                    spanning.is_connected[i][j] = ic > 0.1;
                }
            }

            for i in 0..num_vertices {
                if matrix[i][i] < 0.1 {
                    continue 'test_loop;
                }
            }

            //let dm = DMatrix::from_row_slice(num_vertices, num_vertices, &matrix.as_slice());
            let dm = DMatrix::from_fn(num_vertices, num_vertices, |r, c| matrix[r][c]);

            let det = dm
                .slice((1, 1), (num_vertices - 1, num_vertices - 1))
                .determinant();

            let det2 = spanning.count(num_vertices);

            let mut g: Grid<f64> = Grid::new(num_vertices, num_vertices);
            for i in 0..num_vertices {
                for j in 0..num_vertices {
                    g[(i, j)] = matrix[i][j];
                }
            }
            if det < 0.1 {
                continue;
            }
            println!("{:#.6?}\n{} vs {}", g, det, det2);

            assert_eq!((det + 0.3) as usize, det2);
        }
    }
}
