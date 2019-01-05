

trait ColorTrait {
    const DISP_CHAR: char;
    const INDEX: usize;
    const RYB: u8;

}

struct Red;
struct Orange;
struct Yellow;
struct Green;
struct Blue;
struct Violet;

impl ColorTrait for Red {
    const DISP_CHAR: char = 'R';
    const INDEX: usize = 0;
    const RYB: u8 = 0b0_001_u8; 
}

impl ColorTrait for Orange {
    const DISP_CHAR: char = 'O';
    const INDEX: usize = 1;
    const RYB: u8 = 0b0_011_u8; 
}

impl ColorTrait for Yellow {
    const DISP_CHAR: char = 'Y';
    const INDEX: usize = 2;
    const RYB: u8 = 0b0_010_u8; 
}

impl ColorTrait for Green {
    const DISP_CHAR: char = 'G';
    const INDEX: usize = 3;
    const RYB: u8 = 0b0_110_u8; 
}

impl ColorTrait for Blue {
    const DISP_CHAR: char = 'B';
    const INDEX: usize = 4;
    const RYB: u8 = 0b0_100_u8; 
}

impl ColorTrait for Violet {
    const DISP_CHAR: char = 'V';
    const INDEX: usize = 5;
    const RYB: u8 = 0b0_101_u8; 
}

macro_rules! display_color {
  ( $name:ident ) => {
    impl ::std::fmt::Display for $name {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "{}", $name::DISP_CHAR)
      }
    }
  };
}

display_color!(Red);
display_color!(Orange);
display_color!(Yellow);
display_color!(Green);
display_color!(Blue);
display_color!(Violet);

// R, O(RY), Y, G(YB), B, and V(RB).
fn is_ok<T: ColorTrait, U: ColorTrait>(c1: &T, c2: &U) -> bool
{
    U::RYB & T::RYB == 0
}
