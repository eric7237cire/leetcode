use std::io::stdin;
use std::thread;

type BoardInt = i32;
struct RowCol(BoardInt, BoardInt);

pub fn solve_all_cases()
{
    let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();
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
            debug!("Read chars_line: {:?}", chars_line);
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
            solve(case + 1, n, existing_bishops, existing_rooks)
        }));
    }

    for child in children {
        // collect each child thread's return-value
        println!("{}", child.join().unwrap());
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
    format!("Case #{}: ", case_num)
}

/*

class Board:

    def __init__(self, size):
        self.N = size
        self.rooks = []
        self.bishops = []

        self.existing_bishops = []
        self.existing_rooks = []

        self.board = np.empty(shape = (self.N, self.N))

    def convert_to_tilted_board_coords(self, row, col):
        # https://math.stackexchange.com/questions/383321/rotating-x-y-points-45-degrees
        return  row+col  , col - row + self.N

    def convert_to_board_coords(self, row, col):
        # Kind of guessed this one, looks the translation needs to be spread around too
        return int((row-col)/2 + self.N / 2), int((row+col) / 2 - self.N / 2)

    def create_pivot_board(self):

        self.pivot_board = np.full( shape=(2*self.N, 2*self.N),
                                     fill_value = False,
                                     dtype=np.bool)

        for row in range(0, self.N):
            for col in range(0, self.N):
                # 45 rotation, x+y, y-x
                # and a translation up N to avoid nulls
                coords = self.convert_to_tilted_board_coords(row, col)
                self.pivot_board[coords[0], coords[1]] = True

                check_coords = self.convert_to_board_coords(*coords)
                assert (row,col) == check_coords

        self.board = self.pivot_board


    def write_solution_lines(self):
        ret_str = ""
        n_rows, n_cols = self.board.shape
        for row in range(0, n_rows):
            for col in range(0, n_cols):
                coord = (row,col)
                if coord in self.bishops and \
                        (coord in self.rooks or coord in self.existing_rooks):
                    ret_str += "o" + f" {row+1} {col+1}\n"
                elif coord in self.rooks and coord in self.existing_bishops:
                    ret_str += "o" + f" {row+1} {col+1}\n"
                elif coord in self.rooks:
                    ret_str += "x" + f" {row+1} {col+1}\n"
                elif coord in self.bishops:
                    ret_str += "+" + f" {row+1} {col+1}\n"


        return ret_str
    def solution(self, is_rooks):

        if is_rooks:
            piece_array = self.rooks

            self.board = np.full(shape = (self.N,  self.N),
                                       fill_value = True,
                                       dtype = np.bool)

            for row,col in self.existing_rooks:
                self.board[row] = False
                self.board[:, col] = False

        else:
            piece_array = self.bishops
            self.create_pivot_board()

            for r, c in self.existing_bishops:

                row,col = self.convert_to_tilted_board_coords(r,c)
                self.board[row] = False
                self.board[:, col] = False

        n_rows = self.board.shape[0]
        piece_array.clear()

        for i in range(0, n_rows):
            # Find row with smallest number of empty columns (value 0)
            row_sums = np.sum(self.board, axis = 1)

            # Need to make rows with no spots unattractive
            row_sums[row_sums == 0] = 3 * n_rows
            # Find first free column
            min_row = np.argmin(row_sums)

            min_col = np.argmax(self.board[min_row])

            if self.board[min_row, min_col] == False:
                break

            if is_rooks:
                piece_array.append((min_row, min_col))
            else:
                piece_array.append(self.convert_to_board_coords(min_row, min_col))

            self.board[min_row] = False
            self.board[:,min_col] = False

def solve(case_no, n_str,existing_bishops, existing_rooks):
    print(f"Solving {case_no}")

    b = Board(size = int(n_str))

    b.existing_bishops = existing_bishops
    b.existing_rooks = existing_rooks

    b.solution(is_rooks = True)
    b.solution(is_rooks = False)

    score = len(b.existing_bishops) + len(b.bishops) + \
            len(b.existing_rooks) + len(b.rooks)
    added_pieces = len(set(b.rooks + b.bishops))
    answer_str = f"Case #{case_no}: {score} {added_pieces}\n"

    answer_str += b.write_solution_lines()

    return answer_str

def main():

    #return
    file_base = "small"
    ext = ""
    file_base = "large"
    input_file_name = f"D-{file_base}-practice{ext}.in"
    output_file_name = f"D-{file_base}-practice{ext}.out"

    with open(output_file_name, "w") as output_file, \
            ProcessPoolExecutor(max_workers = 7) as executor, \
            open(input_file_name) as input_file:

        n_cases = int(input_file.readline())
        results = []
        for i in range( n_cases):

            results.append(executor.submit(solve, i+1, n_str, existing_bishops, existing_rooks))

        for r in results:
            ans = r.result()

            output_file.write(ans)


if __name__ == "__main__":
    main()
*/
