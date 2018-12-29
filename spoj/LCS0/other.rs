
/* Returns length of LCS for X[0..m-1], Y[0..n-1] */
fn lcs(  x: &String, y: &String ) -> u16
{ 
    let x = x.as_bytes();
    let y = y.as_bytes();
    let m = x.len();
    let n = y.len();
    let mut len = vec![ vec![0;n+1]; m+1]; 
    //let i:u16;
    //let j:u16; 
   
   /* Following steps build L[m+1][n+1] in bottom up fashion. Note  
      that L[i][j] contains length of LCS of X[0..i-1] and Y[0..j-1] */
   for i in 0..m+1
   { 
     for j in 0..n+1
     { 
       if i == 0 || j == 0 {
         len[i][j] = 0; 
       }
   
       else if x[i-1] == y[j-1] {
         len[i][j] = len[i-1][j-1] + 1;
       } 
   
       else {
         len[i][j] = max(len[i-1][j], len[i][j-1]); 
       }
     } 
   } 

    println!("LCS L matrix");
    for i in 0..m+1
    { 
        println!("{:?}", len[i]);
    }
     
   /* L[m][n] contains length of LCS for X[0..n-1] and Y[0..m-1] */
   return len[m][n]; 
} 


fn debug_print_word( word: &WORD )
    {
        for j in (0..WLEN).rev()
        {
            print!("{:1x}", (word >> (j % WLEN)) & 1);
            if j%WLEN == 0 {
                print!(" ");
            }            
        }
        print!("\n");
    }