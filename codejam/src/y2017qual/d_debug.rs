impl fmt::Display for RowCol
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Board {
#[cfg(feature = "debug_print")]
    fn print_board(&self, board: &BoardVV, is_rooks: bool)
    {
        for (r, row) in board.iter().enumerate() {
            debug!(
                "Row {:2}: {:?}",
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
    fn print_board(_:&BoardVV, _: bool)
    {
        //Do nothing, compiled out
    }
}