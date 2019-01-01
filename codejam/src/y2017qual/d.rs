use std::collections::HashSet;
use std::fmt;
use std::io::stdin;
use std::mem;
use std::thread;

type BoardInt = i32;
#[derive(PartialEq, Debug, Eq, Hash, Clone)]
struct RowCol(BoardInt, BoardInt);

impl fmt::Display for RowCol
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub fn solve_all_cases()
{
    let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();
    //let t = 1;
    for case in 1..=t {
        debug!("Solving case {}", case);

        //handle input / output
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        debug!("Read {}", s);
        let n_and_m: Vec<u32> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let (n, m) = (n_and_m[0], n_and_m[1]);

        // + are bishops
        // x are rooks
        let mut existing_rooks: Vec<RowCol> = Vec::new();
        let mut existing_bishops: Vec<RowCol> = Vec::new();

        for _ in 0..m {
            s.clear();
            stdin().read_line(&mut s).unwrap();
            let chars_line: Vec<&str> = s.split_whitespace().collect();
            //debug!("Read chars_line: {:?}", chars_line);
            let (m_type, row, col): (char, BoardInt, BoardInt) = (
                chars_line[0].chars().next().unwrap(),
                chars_line[1].parse().unwrap(),
                chars_line[2].parse().unwrap(),
            );

            if m_type == 'o' || m_type == 'x' {
                existing_rooks.push(RowCol(row - 1, col - 1));
            }
            if m_type == 'o' || m_type == 'x' {
                existing_bishops.push(RowCol(row - 1, col - 1));
            }
        }

        children.push(thread::spawn(move || -> String {
            solve(case, n, existing_bishops, existing_rooks)
        }));
    }

    for child in children {
        // collect each child thread's return-value
        print!("{}", child.join().unwrap());
    }
    //let ans = solve(nums[0], nums[1]);
    //println!("{} {}", ans.0, ans.1);
}

fn solve(
    case_num: u32,
    n: u32,
    existing_bishops: Vec<RowCol>,
    existing_rooks: Vec<RowCol>,
) -> String
{
    debug!("Solving case {}", case_num);

    let mut b = Board::new(n as BoardInt);

    b.existing_bishops = existing_bishops;
    b.existing_rooks = existing_rooks;

    b.solution(true);
    b.solution(false);

    let score = b.existing_bishops.len() + b.bishops.len() + b.existing_rooks.len() + b.rooks.len();

    let mut added_pieces: HashSet<RowCol> = b.rooks.iter().map(|rc| rc.clone()).collect();
    added_pieces.extend(b.bishops.iter().map(|rc| rc.clone()));

    let mut answer_str = format!("Case #{}: {} {}\n", case_num, score, added_pieces.len());

    answer_str += &b.write_solution_lines();

    return answer_str;
}

struct Board
{
    N: BoardInt,
    rooks: Vec<RowCol>,
    bishops: Vec<RowCol>,

    existing_bishops: Vec<RowCol>,
    existing_rooks: Vec<RowCol>,

    board: Vec<Vec<bool>>,
    pivot_board: Vec<Vec<bool>>,
}

impl Board
{
    fn new(size: BoardInt) -> Board
    {
        Board {
            N: size,
            rooks: Vec::new(),
            bishops: Vec::new(),

            existing_bishops: Vec::new(),
            existing_rooks: Vec::new(),

            board: vec![vec![false; size as usize]; size as usize],
            pivot_board: Vec::new(),
        }
    }

    fn convert_to_tilted_board_coords(&self, row: BoardInt, col: BoardInt) -> RowCol
    {
        // https://math.stackexchange.com/questions/383321/rotating-x-y-points-45-degrees
        RowCol(row + col, col - row + self.N)
    }

    fn convert_to_board_coords(&self, row: BoardInt, col: BoardInt) -> RowCol
    {
        // Kind of guessed this one, looks the translation needs to be spread around too
        RowCol(((row - col) + self.N) / 2, ((row + col) - self.N) / 2)
    }

    fn convert_to_board_coords_opt(&self, row: BoardInt, col: BoardInt) -> Option<RowCol>
    {
        if ((row - col) + self.N) % 2 != 0 {
            return None;
        }
        if ((row + col) - self.N) % 2 != 0 {
            return None;
        }
        // Kind of guessed this one, looks the translation needs to be spread around too
        let ret = RowCol(((row - col) + self.N) / 2, ((row + col) - self.N) / 2);

        if ret.0 < 0 || ret.0 >= self.N {
            return None;
        }
        if ret.1 < 0 || ret.1 >= self.N {
            return None;
        }
        return Some(ret);
    }

    fn create_pivot_board(&mut self)
    {
        self.pivot_board = vec![vec![false; 2 * self.N as usize]; 2 * self.N as usize];

        for row in 0..self.N {
            for col in 0..self.N {
                // 45 rotation, x+y, y-x
                // and a translation up N to avoid nulls
                let coords = self.convert_to_tilted_board_coords(row, col);
                //Only pivot-able coordinates are open/true
                self.pivot_board[coords.0 as usize][coords.1 as usize] = true;

                let check_coords = self.convert_to_board_coords(coords.0, coords.1);
                assert_eq!(RowCol(row, col), check_coords);
            }
        }

        mem::swap(&mut self.board, &mut self.pivot_board);
        self.pivot_board.clear();
        //self.board = self.pivot_board;
    }

    fn write_solution_lines(&self) -> String
    {
        let mut ret_str = String::new();
        let n_rows = self.board.len();
        let n_cols = self.board[0].len();
        for row in 0..n_rows {
            for col in 0..n_cols {
                let coord = RowCol(row as BoardInt, col as BoardInt);
                let rc_str = format!(" {} {}\n", row + 1, col + 1);

                if self.bishops.contains(&coord)
                    && (self.rooks.contains(&coord) || self.existing_rooks.contains(&coord))
                {
                    ret_str += "o";
                    ret_str += &rc_str;
                } else if self.rooks.contains(&coord) && self.existing_bishops.contains(&coord) {
                    ret_str += "o";
                    ret_str += &rc_str;
                } else if self.rooks.contains(&coord) {
                    ret_str += "x";
                    ret_str += &rc_str;
                } else if self.bishops.contains(&coord) {
                    ret_str += "+";
                    ret_str += &rc_str;
                }
            }
        }

        ret_str
    }

    fn set_col(&mut self, col: usize, v: bool)
    {
        for r in 0usize..self.board.len() {
            self.board[r][col] = v;
        }
    }

    fn set_row(&mut self, row: usize, v: bool)
    {
        for c in 0usize..self.board.len() {
            self.board[row][c] = v;
        }
    }
 
    #[cfg(feature="debug_print")]
    fn print_board(&self, is_rooks: bool)
    {        
        for (r, row) in self.board.iter().enumerate() {
            debug!(
                "Row {}: {:?}",
                r,
                row.iter()
                    .enumerate()
                    .map(|(c, b)| {
                        let check = is_rooks
                            || None
                                != self.convert_to_board_coords_opt(r as BoardInt, c as BoardInt);
                        if !check {
                            return "#";
                        }

                        if *b {
                            "."
                        } else {
                            "O"
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            );
        }
    }

    #[cfg(not(feature = "debug_print"))]
    fn print_board(&self, is_rooks: bool)
    {
    }

    fn solution(&mut self, is_rooks: bool)
    {
        if is_rooks {
            self.board = vec![vec![true; self.N as usize]; self.N as usize];
            self.print_board(is_rooks);
            let rooks_clone = self.existing_rooks.clone();
            let rook_it = rooks_clone.iter();
            for RowCol(row, col) in rook_it {
                self.set_row(*row as usize, false);
                self.set_col(*col as usize, false);
            }
            debug!("After placing {} existing rooks", self.existing_rooks.len());
            self.print_board(is_rooks);
        } else {
            self.create_pivot_board();

            let bishops_clone = self.existing_bishops.clone();
            for RowCol(row, col) in bishops_clone.iter() {
                let t_rc = self.convert_to_tilted_board_coords(*row, *col);

                self.set_row(t_rc.0 as usize, false);
                self.set_col(t_rc.1 as usize, false);
            }
            debug!(
                "After placing {} existing bishops",
                self.existing_bishops.len()
            );
            self.print_board(is_rooks);
        }

        let n_rows = self.board[0].len();
        let mut piece_array: Vec<RowCol> = Vec::new();

        for index in 0..n_rows {
            // Find row with smallest number of empty columns (value 0)
            let mut row_sums: Vec<usize> = self
                .board
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

            debug!("Row sums: {:?}", row_sums);

            // Need to make rows with no spots unattractive
            for ri in row_sums.iter_mut().filter(|rs| **rs == 0) {
                *ri = 3 * n_rows;
            }

            //find row with smallest # of free columns (free=true, taken = false)
            let min_row = row_sums
                .iter()
                .enumerate()
                .map(|(x, y)| (y, x))
                .min()
                .unwrap()
                .1;

            // Find first free column (free=true/1)

            let min_col = (self.board[min_row]
                .iter()
                .enumerate()
                //-idx to get the first column
                .map(|(idx, is_free) | (is_free, -(idx as i32)))
                .max()
                .unwrap()
                .1 * -1) as usize;

            if self.board[min_row][min_col] == false {
                break;
            }

            if is_rooks {
                piece_array.push(RowCol(min_row as BoardInt, min_col as BoardInt));
            } else {
                piece_array
                    .push(self.convert_to_board_coords(min_row as BoardInt, min_col as BoardInt));
            }
            self.set_row(min_row, false);
            self.set_col(min_col, false);
            debug!(
                "After processing row {}.  Placed at {},{}",
                index, min_row, min_col
            );
            self.print_board(is_rooks);
        }

        {
            if is_rooks {
                self.rooks = piece_array;
            } else {
                self.bishops = piece_array;
            }
        }
    }
}
