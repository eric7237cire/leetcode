use crate::util::codejam::run_cases;
use crate::util::grid::Grid;
use nalgebra::*;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::StdRng;
use rand::SeedableRng;
use rulinalg::matrix::Matrix;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::Instant;

const MAX_VERTEX: usize = 22;
const K: usize = 10000;

/*
Counting spanning trees
Searching
Gaussian elimination
Determinant of a matrix
*/

pub fn solve_all_cases()
{
    let mut spanning = Spanning::new();
    let mut search = Search::new();

    search.dfs(&mut spanning, 1, 0);

    run_cases(&["C-small-practice"], "y2017round4", |reader, buffer| {
        let t = reader.read_int();

        for case in 1..=t {
            let k = reader.read_int();

            write!(
                buffer,
                "Case #{}: {}\n{}\n",
                case,
                search.mp[&k].len(),
                search.mp[&k].join("\n")
            )
            .unwrap();
        }
    });
}

struct Spanning
{
    is_connected: [[bool; MAX_VERTEX]; MAX_VERTEX],
    gauss_matrix: [[f64; MAX_VERTEX]; MAX_VERTEX],
}

impl Spanning
{
    fn new() -> Self
    {
        Spanning {
            is_connected: [[false; MAX_VERTEX]; MAX_VERTEX],
            gauss_matrix: [[0f64; MAX_VERTEX]; MAX_VERTEX],
        }
    }

    fn spanning_tree_count(&mut self, num_vertices: usize) -> usize
    {
        for i in 0..num_vertices {
            for j in 0..=i {
                self.gauss_matrix[i][j] = 0.0;
                self.gauss_matrix[j][i] = 0.0;
            }
        }

        for i in 0..num_vertices {
            for j in 0..i {
                let ic = if self.is_connected[i][j] { 1f64 } else { 0f64 };

                self.gauss_matrix[i][j] = -ic;
                self.gauss_matrix[j][i] = -ic;

                self.gauss_matrix[i][i] += ic;
                self.gauss_matrix[j][j] += ic;
            }
        }

        //gaussian elimination
        let mut n = num_vertices;
        n -= 1;
        for e in 0..n {
            //assert(fabs(b[e][e]) > eps);
            for i in e + 1..n {
                let coeff = -self.gauss_matrix[i][e] / self.gauss_matrix[e][e];
                for j in e..n {
                    self.gauss_matrix[i][j] += coeff * self.gauss_matrix[e][j];
                }
            }
        }

        let mut ans = 1.0;
        for i in 0..n {
            ans *= self.gauss_matrix[i][i];
        }

        (ans + 0.5) as usize
    }
}

struct Search
{
    now: Instant,
    mp: HashMap<usize, Vec<String>>,
    mpn: HashSet<(usize, usize)>,
}

impl Search
{
    fn new() -> Self
    {
        Search {
            now: Instant::now(),
            mp: HashMap::new(),
            mpn: HashSet::new(),
        }
    }
    /*
        final search tree looks like
        DFS 1
    DFS 2
    DFS 3
    DFS 4
    DFS 5
    DFS 6
    DFS 7
    DFS 8
    DFS 9
    DFS 10
    DFS 11
    DFS 12
    DFS 13
    DFS 14
    DFS 15
    DFS 16
    DFS 17
    DFS 18
    DFS 19
    DFS 20
    DFS 21
    DFS 21
    DFS 21
    DFS 21
    DFS 21
    DFS 21
    DFS 21
    DFS 21
    */
    fn dfs(&mut self, spanning: &mut Spanning, v: usize, level: usize)
    {
        if (self.mp.len() == K - 1) {
            return;
        }
        if (v == MAX_VERTEX) {
            return;
        }
        //println!("DFS {}", v);

        //try connecting v to all vertices less than it
        for v_connected_perm in 1..(1 << v) {
            if (self.mp.len() == K - 1) {
                return;
            }
            for i in 0..v {
                spanning.is_connected[v][i] = if v_connected_perm & (1 << (v - i - 1)) > 0 {
                    true
                } else {
                    false
                };
            }
            let cnt = spanning.spanning_tree_count(v + 1);
            if (cnt > K) {
                continue;
            }
            if !self.mp.contains_key(&cnt) {
                let mut z: Vec<String> = (0..=v)
                    .map(|i| {
                        (0..=v)
                            .map(|j| {
                                if spanning.is_connected[max(i, j)][min(i, j)] {
                                    '1'
                                } else {
                                    '0'
                                }
                            })
                            .collect::<String>()
                    })
                    .collect();

                self.mp.insert(cnt, z);
                println!(
                    "found cnt = {} with n = {}; time = {} secs ; mp.size() = {}; level = {}",
                    cnt,
                    v + 1,
                    self.now.elapsed().as_secs(),
                    self.mp.len(),
                    level
                );
            }
            //only try different vertices for here-to-for unseen values of k
            if !self.mpn.contains(&(cnt, v + 1)) {
                self.mpn.insert((cnt, v + 1));
                self.dfs(spanning, v + 1, 1 + level);
            }
        }
    }
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

            let det2 = spanning.spanning_tree_count(num_vertices);

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
