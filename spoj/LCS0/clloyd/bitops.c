
#include <stdio.h>
/*** #include "bits.h" ***/
#define	WLEN	  32	/* word length */
#define	LOGWLEN	  5	/* log word length -- round(LOG2(WLEN) */
#define SMAX	  4096	/* maximum string length -- multiple of WLEN */
#define BITMAX	  128	/* maximum bit string length -- round(SMAX/WLEN) */
#define	ALPHASIZE 4	/* alphabet size */

typedef unsigned int WORD;
typedef short unsigned int INDEX;	/* i: alpha[i] == char */

extern char alpha[];	/* alphabet */

extern void randomstring();
extern int lcs();
extern void alphastrings();
extern void bitops();
extern void simpleops();
extern int bitcount();

extern int nwords;				/* no. of words for bits */
extern WORD a_strings[ALPHASIZE][BITMAX];

void bitops(last, cur, index)
WORD *last, *cur;
INDEX index;
{
	register WORD x, y;
	register int j;
	register WORD *a_s;
	register WORD top_borrow, bottombit;

	a_s = &a_strings[index][0];
	bottombit = 1;
	for (j = 0; j < nwords; j++) {
		y = *(last++);
		x =  y | *(a_s++);
		top_borrow = (y >> (WLEN - 1)) & 0x1;
		y = ((y << 1) | bottombit);
		if (x < y)
			top_borrow = 1;
		*(cur++) = x & ((x - y) ^ x);
		bottombit = top_borrow;
	}
	return;
}

/* The bit-string LCS program, i.e the *.c and *.h files, in this subdirectory,
   is released under the "GNU General Public License" (GPL) Version 2,
   June 1991, [ http://www.gnu.org/copyleft/gpl.html ]  provided that
   any resulting publications refer to the following paper: 
         L. Allison and T.I. Dix (1986).
         A Bit-String Longest-Common-Subsequence Algorithm.
         Inf. Proc.  Lett., V23, Dec' 1986, pp305-310. 
   - L. Allison & T. I. Dix, 5/2001
*/
/* NB. 1986 "classic" C, Vax 11-750,  no warranty! */

