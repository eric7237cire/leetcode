
#include <stdio.h>
#include "bits.h"

int simple1[SMAX+1];	/* buffers for simple algorithm */
int simple2[SMAX+1];
int *ps1, *ps2, *t;		/* pointers to same */

int simplelcs(a, lena, b, lenb)
INDEX *a;
int lena;
INDEX *b;
int lenb;
{
	register int i;

	ps1 = simple1;
	for (i = 0; i <= lenb; i++)
		*(ps1++) = 0;		/*** simple1[i] = 0; ***/
	simple2[0] = 0;
	ps1 = simple1; ps2 = simple2;

/*** fprintf(stderr, "Simple intialization done\n"); ***/
	for (i = 0; i < lenb; i++) {
		simpleops(ps1, ps2, i, a, b, lena);
/*** fprintf(stderr, "Simpleops done\n"); ***/
		t = ps1; ps1 = ps2; ps2 = t;
	}
	return (ps1[lena]);
}

void simpleops(last, cur, i, a, b, lena)
int *last, *cur, i;
INDEX *a, *b;
int lena;
{
	register int j, max;
	register INDEX c;

	c = b[i];
	for (j = 0; j < lena; j++) {
		if (a[j] == c)
			max = last[j] + 1;
		else
			max = 0;
		if (max < last[j+1])
			max = last[j+1];
		if (max < cur[j])
			max = cur[j];
		cur[j+1] = max;
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

