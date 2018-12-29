use std::cmp::{max};
use std::collections::HashMap;
use std::io::{self, BufRead};

/* Returns length of LCS for X[0..m-1], Y[0..n-1] */
fn lcs(  X: &String, Y: &String ) -> u16
{ 
    let X = X.as_bytes();
    let Y = Y.as_bytes();
    let m = X.len();
    let n = Y.len();
    let mut L = vec![ vec![0;n+1]; m+1]; 
    let i:u16;
    let j:u16; 
   
   /* Following steps build L[m+1][n+1] in bottom up fashion. Note  
      that L[i][j] contains length of LCS of X[0..i-1] and Y[0..j-1] */
   for i in 0..m+1
   { 
     for j in 0..n+1
     { 
       if i == 0 || j == 0 {
         L[i][j] = 0; 
       }
   
       else if X[i-1] == Y[j-1] {
         L[i][j] = L[i-1][j-1] + 1;
       } 
   
       else {
         L[i][j] = max(L[i-1][j], L[i][j-1]); 
       }
     } 
   } 
     
   /* L[m][n] contains length of LCS for X[0..n-1] and Y[0..m-1] */
   return L[m][n]; 
} 

const 	WLEN: usize	=  32;	/* word length */
const 	LOGWLEN: usize=	  5;	/* log word length -- round(LOG2(WLEN) */
const  SMAX: usize	 = 50016;	/* maximum string length -- multiple of WLEN */
const BITMAX: usize	=  1563;	/* maximum bit string length -- round(SMAX/WLEN) */

const 	ALPHASIZE: usize = 4;	/* alphabet size */
type WORD = u32;
static ALPHA : [char;ALPHASIZE] = ['a', 'c', 'g', 't'];

pub struct StringUtil 
{
    alpha: Vec<char>,
    c_to_index: HashMap<char,usize>
}

impl StringUtil 
{
    pub fn init<'a, I>(p_alpha: I) -> StringUtil
    where I: Iterator<Item = &'a char>
    {
        let my_alpha:Vec<char> = p_alpha.map(|x| x.clone()).collect();
        let mut my_c_to_index: HashMap<char,usize> = HashMap::new();
        for (i, c) in my_alpha.iter().enumerate()
        {
            my_c_to_index.insert(*c, i);
        }

        StringUtil{ alpha: my_alpha, c_to_index: my_c_to_index }
    }

    fn to_index_vec<I>(&self, chars: I) -> Vec<usize>
    where I: Iterator<Item = char>
    {
        return chars.map( |c| self.c_to_index[&c]).collect();
    }
}


#[test]
fn test_string_util() {
    let s = StringUtil::init( ['c', 'd', 'a'].iter() );

    let v1 = s.to_index_vec( "ddaaccad".chars() );
    // not real lex distance but its ok for our needs
    assert_eq!(v1, vec![1,1,2,2,0,0,2,1]);
}

//['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p','q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];	/* alphabet */

//static mut nwords: usize = 0;

fn alphastrings(s: Vec<usize>, a_strings : &mut [[u32;BITMAX];ALPHASIZE]) 
{
    println!("S length={}", s.len());
    let nwords = (s.len() + WLEN - 1) / WLEN;
	//register INDEX *p;
	let i:usize;
    let j:usize;

	for i in 0..ALPHASIZE
    {
		for j in 0..nwords
        {
			a_strings[i][j] = 0;
        }
    }
	
	for (i, p) in s.iter().enumerate()
    {
		a_strings[*p as usize][i/WLEN] |=  1 << (i % WLEN);
    }

/*** debug ***/
	for i in 0..ALPHASIZE {
		println!("%c-string : {}", ALPHA[i]);
		for j in (0..s.len()).rev()
        {
			if j%WLEN == 0 {
				print!("{:1x} ", (a_strings[i][j/WLEN] >> (j % WLEN)) & 1);
            }
			else {
				print!("{:1x}", (a_strings[i][j/WLEN] >> (j % WLEN)) & 1);
            }
        }
		println!("\n");
	}
/*** debug ended ***/

	return;
}



fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    //let mut iterator = io::stdin().lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    let line2 = iterator.next().unwrap().unwrap();
    let ans = lcs(&line1, &line2);
    println!("{}", ans);

    let mut c_to_index = HashMap::new();
    for (i, c) in ALPHA.iter().enumerate()
    {
        c_to_index.insert(c, i);
    }

    println!("String A: {}", line1);
    let str1: Vec<usize> = line1.chars().map( |c| c_to_index[&c]).collect();
    println!("String A: {:?}", str1);

    let mut a_strings : [[u32;BITMAX];ALPHASIZE] = [[0;BITMAX];ALPHASIZE] ;
    alphastrings(str1, &mut a_strings);
}