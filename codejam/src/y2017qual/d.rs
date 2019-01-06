use std::io::stdin;
use std::thread;

type BoardInt = i32;
type BoardVV = Vec<Vec<bool>>;
#[derive(PartialEq, Debug, Eq, Hash, Clone)]
struct RowCol(BoardInt, BoardInt);

pub fn solve_all_cases()
{
    let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();

    for case in 1..=t {
        //handle input / output
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        //debug!("Read {}", s);
        let n_and_m: Vec<u32> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let (n, m) = (n_and_m[0], n_and_m[1]);

        /* + are bishops
         * x are rooks; place as many as possible such that no row has 2
         * It is a bit confusing to understand that from the restriction
         * Whenever any two models share a diagonal of the grid, at least one of the two must be an x
         *
         * We basically place as many as possible of each indepedently, then combine them to 'o'
         * we pivot the bishop boards to make it the same problem as rooks
         */
        let mut existing_rooks: Vec<RowCol> = Vec::new();
        let mut existing_bishops: Vec<RowCol> = Vec::new();

        for _ in 0..m {
            s.clear();
            stdin().read_line(&mut s).unwrap();
            let chars_line: Vec<&str> = s.split_whitespace().collect();

            let (m_type, row, col): (char, BoardInt, BoardInt) = (
                chars_line[0].chars().next().unwrap(),
                chars_line[1].parse().unwrap(),
                chars_line[2].parse().unwrap(),
            );

            if m_type == 'o' || m_type == 'x' {
                existing_rooks.push(RowCol(row - 1, col - 1));
            }
            if m_type == 'o' || m_type == '+' {
                existing_bishops.push(RowCol(row - 1, col - 1));
            }
        }

        if cfg!(feature = "debug_print") && case != 4 {
            continue;
        }

        children.push(thread::spawn(move || -> String {
            solve(case, n, existing_bishops, existing_rooks)
        }));
    }

    for child in children {
        // collect each child thread's return-value
        print!("{}", child.join().unwrap());
    }
}

fn solve(
    case_num: u32,
    n: u32,
    existing_bishops: Vec<RowCol>,
    existing_rooks: Vec<RowCol>,
) -> String
{
    let b = Board::new(n as BoardInt, existing_bishops, existing_rooks);

    let rooks = b.add_pieces(true);
    let bishops = b.add_pieces(false);

    let score = b.existing_bishops.len() + bishops.len() + b.existing_rooks.len() + rooks.len();

    let (lines, added) = b.write_solution_lines(&rooks, &bishops);
    format!("Case #{}: {} {}\n{}", case_num, score, added, lines)
}

struct Board
{
    n: BoardInt,
    existing_bishops: Vec<RowCol>,
    existing_rooks: Vec<RowCol>,
}

impl Board
{
    fn new(n: BoardInt, existing_bishops: Vec<RowCol>, existing_rooks: Vec<RowCol>) -> Board
    {
        Board {
            n,
            existing_bishops,
            existing_rooks,
        }
    }

    fn convert_to_tilted_board_coords(&self, row: BoardInt, col: BoardInt) -> RowCol
    {
        // https://math.stackexchange.com/questions/383321/rotating-x-y-points-45-degrees
        RowCol(row + col, col - row + self.n)
    }

    fn convert_to_board_coords(&self, row: BoardInt, col: BoardInt) -> RowCol
    {
        // Kind of guessed this one, looks the translation needs to be spread around too
        RowCol(((row - col) + self.n) / 2, ((row + col) - self.n) / 2)
    }

    fn create_pivot_board(&self) -> BoardVV
    {
        let mut board = vec![vec![false; 2 * self.n as usize]; 2 * self.n as usize];

        for row in 0..self.n {
            for col in 0..self.n {
                // 45 rotation, x+y, y-x
                // and a translation up N to avoid nulls
                let coords = self.convert_to_tilted_board_coords(row, col);
                //Only pivot-able coordinates are open/true
                board[coords.0 as usize][coords.1 as usize] = true;

                let check_coords = self.convert_to_board_coords(coords.0, coords.1);
                assert_eq!(RowCol(row, col), check_coords);
            }
        }

        board
    }

    fn write_solution_lines(&self, rooks: &Vec<RowCol>, bishops: &Vec<RowCol>) -> (String, usize)
    {
        let mut ret_str = String::new();
        let mut added = 0;

        for row in 0..self.n {
            for col in 0..self.n {
                let coord = RowCol(row as BoardInt, col as BoardInt);
                let len_before = ret_str.len();

                if bishops.contains(&coord)
                    && (rooks.contains(&coord) || self.existing_rooks.contains(&coord))
                {
                    ret_str += "o";
                } else if rooks.contains(&coord) && self.existing_bishops.contains(&coord) {
                    ret_str += "o";
                } else if rooks.contains(&coord) {
                    ret_str += "x";
                } else if bishops.contains(&coord) {
                    ret_str += "+";
                }

                if ret_str.len() > len_before {
                    ret_str += &format!(" {} {}\n", row + 1, col + 1);
                    added += 1
                }
            }
        }

        (ret_str, added)
    }

    fn set_col(&self, board: &mut BoardVV, col: usize, v: bool)
    {
        for r in 0usize..board.len() {
            board[r][col] = v;
        }
    }

    fn set_row(&self, board: &mut BoardVV, row: usize, v: bool)
    {
        for c in 0usize..board.len() {
            board[row][c] = v;
        }
    }

    fn add_pieces(&self, is_rooks: bool) -> Vec<RowCol>
    {
        let mut board: BoardVV;
        if is_rooks {
            board = vec![vec![true; self.n as usize]; self.n as usize];

            for RowCol(row, col) in self.existing_rooks.iter() {
                self.set_row(&mut board, *row as usize, false);
                self.set_col(&mut board, *col as usize, false);
            }
        } else {
            board = self.create_pivot_board();
            for RowCol(row, col) in self.existing_bishops.iter() {
                let t_rc = self.convert_to_tilted_board_coords(*row, *col);

                self.set_row(&mut board, t_rc.0 as usize, false);
                self.set_col(&mut board, t_rc.1 as usize, false);
            }
        }

        let n_rows = board[0].len();
        let mut piece_array: Vec<RowCol> = Vec::new();

        for index in 0..n_rows {
            // Find row with smallest number of empty columns (value 0)
            let row_sums: Vec<usize> = board
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|b| match b {
                            true => 1,
                            false => 0,
                        })
                        .sum()
                })
                .collect();

            debug!("Row sums (len={}): {:?}", row_sums.len(), row_sums);

            //find row with smallest # of free columns (free=true, taken = false)
            let min_row_opt = row_sums
                .iter()
                .enumerate()
                .map(|(idx, y)| (y, idx))
                .filter(|&(row_sum, _)| *row_sum > 0) //must have at least one free spot
                .min();

            if min_row_opt.is_none() {
                break; //we can't place anymore pieces
            }

            let min_row = min_row_opt.unwrap().1;

            // Find first free column (free=true/1)
            let min_col = board[min_row].iter().position(|b| *b).unwrap();

            piece_array.push(if is_rooks {
                (RowCol(min_row as BoardInt, min_col as BoardInt))
            } else {
                self.convert_to_board_coords(min_row as BoardInt, min_col as BoardInt)
            });

            self.set_row(&mut board, min_row, false);
            self.set_col(&mut board, min_col, false);

            debug!(
                "After processing row {}.  Placed at {},{}",
                index, min_row, min_col
            );
            self.print_board(&board, is_rooks);
        }

        piece_array
    }
}

include!("d_debug.rs");
