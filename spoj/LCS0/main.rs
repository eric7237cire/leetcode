use std::cmp::{max};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::mem;
use std::u32;

const 	WLEN: usize	=  32;	/* word length */
const 	LOGWLEN: usize=	  5;	/* log word length -- round(LOG2(WLEN) */
const  SMAX: usize	 = 50016;	/* maximum string length -- multiple of WLEN */
//const BITMAX: usize	=  1563;	/* maximum bit string length -- round(SMAX/WLEN) */

//const 	ALPHASIZE: usize = 4;	/* alphabet size */
type WORD = u32;


pub struct BitLCS 
{
    alpha: Vec<char>,
    c_to_index: HashMap<char,usize>,
    a_strings : Vec<Vec<WORD>>,
    a_len: usize
}

//http://users.monash.edu/~lloyd/tildeStrings/Alignment/86.IPL.html
impl BitLCS 
{
    pub fn init<'a, I>(p_alpha: I) -> BitLCS
    where I: Iterator<Item = &'a char>
    {
        let my_alpha:Vec<char> = p_alpha.map(|x| x.clone()).collect();
        let mut my_c_to_index: HashMap<char,usize> = HashMap::new();
        for (i, c) in my_alpha.iter().enumerate()
        {
            my_c_to_index.insert(*c, i);
        }

        BitLCS{ 
            alpha: my_alpha, 
            c_to_index: my_c_to_index, 
            a_strings: Vec::new(),
            a_len: 0 }
    }

    fn to_index_vec<I>(&self, chars: I) -> Vec<usize>
    where I: Iterator<Item = char>
    {
        return chars.map( |c| self.c_to_index[&c]).collect();
    }

    ///Initializes a_strings with the given string
    fn alphastrings(&mut self, s: &[usize]) 
    {
        //println!("S length={}", s.len());
        self.a_len = s.len();
        let nwords = (s.len() + WLEN - 1) / WLEN;
        
        for i in 0..self.alpha.len()
        {
            self.a_strings.push( vec![0; nwords] );
        }
        
        for (i, p) in s.iter().enumerate()
        {
            self.a_strings[*p as usize][i/WLEN] |=  1 << (i % WLEN);
        }


    /*** debug ***/
    /*
        for i in 0..self.alpha.len() {
            println!("%c-string : {}", self.alpha[i]);
            self.debug_print(&self.a_strings[i]);            
        }
        */
    /*** debug ended ***/

        return;
    }


    fn bitops(&self, last:&[WORD], cur: &mut [WORD], index: usize)
    {
        let mut x: WORD;
        let mut y: WORD;
        //let j;
        //let mut a_s: &WORD;
        let mut top_borrow;
        let mut bottombit :WORD;

        let nwords = self.a_strings[0].len();

        /*println!("BitOps.  Prev row");
        self.debug_print(last);
        println!("BitOps.  Cur row");
        self.debug_print(last);*/

        //a_s = &a_strings[index][0];
        bottombit = 1;
        //for (j = 0; j < nwords; j++) {
        for(j, a_s) in self.a_strings[index].iter().enumerate()
        {
            y = last[j];
            x =  y | *a_s;
            top_borrow = (y >> (WLEN - 1)) & 0x1;
            y = (y << 1) | bottombit;
            if x < y {
                top_borrow = 1;
            }
            //simulate C overflow, sheesh
            let x_minus_y = if x >= y { x-y } else { u32::MAX -y + x +1} ;

            cur[j] = x & (x_minus_y ^ x);
            bottombit = top_borrow;
        }
        return;
    }

    fn debug_print(&self, words: &[WORD] )
    {
        for j in (0..self.a_len).rev()
        {
            print!("{:1x}", (words[j/WLEN] >> (j % WLEN)) & 1);
            if j%WLEN == 0 {
                print!(" ");
            }            
        }
        print!("\n");
    }

    

}

const bitmask : [WORD;LOGWLEN] =
		[0x55555555u32, 0x33333333u32, 0x0f0f0f0fu32, 0x00ff00ffu32, 0x0000ffffu32];
fn bitcount(wp: &[WORD]) -> usize
{
	//register WORD w, count;
	//register int j, rshift, i;

	let mut count:usize = 0;
	for word in wp {
		
		let mut rshift = 1;
        let mut w = *word;
		for j in 0..LOGWLEN {
			w = (w & bitmask[j]) + ((w & !bitmask[j]) >> rshift);
			rshift <<= 1;
		}
		count += w as usize;
	}
	return count;
}

#[test]
fn test_string_util() {
    let mut s = BitLCS::init( ['c', 'd', 'a'].iter() );

    let v1 = s.to_index_vec( "ddaaccad".chars() );
    // not real lex distance but its ok for our needs
    assert_eq!(v1, vec![1,1,2,2,0,0,2,1]);

    s.alphastrings(&v1);

    assert_eq!(3, s.a_strings.len());
    assert_eq!(1<<5, 0b0010_0000_u8);
    assert_eq!(1<<5, 32);
    assert_eq!(11, 0b0000_1011_u8);

    //a's, note the order is reversed, the left/most sig bit matches the last index (which is the right)
    assert_eq!(0b_01001100_u32, s.a_strings[2][0]);
    //d
    assert_eq!(0b_10000011_u32, s.a_strings[1][0]);
    //c
    assert_eq!(0b_00110000_u32, s.a_strings[0][0]);
    
    
}

//['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p','q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];	/* alphabet */

//static mut nwords: usize = 0;




fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    //let mut iterator = io::stdin().lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    let line2 = iterator.next().unwrap().unwrap();
    //let ans = lcs(&line1, &line2);
    //println!("{}", ans);

    
    // let alpha : [char;4] = ['a', 'c', 'g', 't'];
    let alpha : [char;26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p','q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];	/* alphabet */
    let mut lcs = BitLCS::init(alpha.iter());
    
    //println!("String A: {:?}", line1.chars().rev().collect::<String>());
    let str1: Vec<usize> = lcs.to_index_vec(line1.chars().rev());
    let str2: Vec<usize> = lcs.to_index_vec(line2.chars().rev());
    //println!("String A: {:?}", str1);

    lcs.alphastrings(&str1);

    let mut bit1 = vec![0; lcs.a_strings[0].len()];
    let mut bit2 = vec![0; lcs.a_strings[0].len()];

	for (i, b_char_index) in str2.iter().enumerate()
    {
		lcs.bitops(&bit1, &mut bit2, *b_char_index);
/***/
		/*print!("row[{:2}] :   ", i+1);
		lcs.debug_print(&bit2);
        assert!(i != 0 || bit2[0] == 0b_0000010000_u32);
        assert!(i != 1 || bit2[0] == 0b_0010010000_u32);
        assert!(i != 2 || bit2[0] == 0b_1000100100_u32);
        assert!(i != 3 || bit2[0] == 0b_0010010100_u32);
        assert!(i != 4 || bit2[0] == 0b_0110001001_u32);

        assert!(i != 5 || bit2[0] == 0b_0110001001_u32);
        assert!(i != 6 || bit2[0] == 0b_0110000011_u32);*/
/***/

		mem::swap(&mut bit1, &mut bit2);
	}

    let bit_count = bitcount(&bit1);
    println!("{}", bit_count);
}