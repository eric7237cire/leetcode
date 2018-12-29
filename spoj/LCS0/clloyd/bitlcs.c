
#include <stdio.h>
#include "bits.h"

int nwords;				/* no. of words for bits */
WORD bit1[BITMAX];
WORD bit2[BITMAX];
WORD a_strings[ALPHASIZE][BITMAX];
WORD *pb1, *pb2, *t1;

WORD bitmask[LOGWLEN] =
		{0x55555555, 0x33333333, 0x0f0f0f0f, 0x00ff00ff, 0x0000ffff};

int bitlcs(a, lena, b, lenb)
INDEX *a;
int lena;
INDEX *b;
int lenb;
{
	register int i;
	register INDEX *pbstring;
	register int j, k;

	nwords = (lena + WLEN - 1) / WLEN;

	alphastrings(a, lena);
/*** fprintf(stderr, "alphastrings done\n"); ***/

	pb1 = &bit1[0];
	for (i = 0; i < nwords; i++)
		*pb1++ = 0;		/*** bit1[i] = 0; ***/
	pb1 = &bit1[0]; pb2 = &bit2[0];
	pbstring = b;

	for (i = 1; i <= lenb; i++) {
		bitops(pb1, pb2, *(pbstring++));
/*** fprintf(stderr, "bitops done\n"); ***/

/***/
		printf("row[%2d] :   ", i);
		for (j = lena - 1; j >= 0; j--) {
			k = (pb2[j/WLEN] >> (j % WLEN)) & 1;
			if (j%WLEN == 0)
				printf("%1x ", k);
			else
				printf("%1x", k);
		}
		printf("\n");
/***/

		t1 = pb1; pb1 = pb2; pb2 = t1;
	}
	return (bitcount(pb1));
}

void alphastrings(s, len)
INDEX *s;
int len;
{
	register INDEX *p;
	register int i, j;

	for (i = 0; i < ALPHASIZE; i++)
		for (j = 0; j < nwords; j++)
			a_strings[i][j] = 0;
	p =  s;
	j = len;
	for (i = 0; i < j; i++)
		a_strings[*(p++)][i/WLEN] |=  1 << (i % WLEN);

/*** debug ***/
	for (i = 0; i < ALPHASIZE; i++) {
		printf("%c-string :  ", alpha[i]);
		for (j = len - 1; j >= 0; j--)
			if (j%WLEN == 0)
				printf("%1x ", (a_strings[i][j/WLEN] >> (j % WLEN)) & 1);
			else
				printf("%1x", (a_strings[i][j/WLEN] >> (j % WLEN)) & 1);
		printf("\n");
	}
/*** debug ended ***/

	return;
}

int bitcount(wp)
WORD *wp;
{
	register WORD w, count;
	register int j, rshift, i;

	count = 0;
	for (i = 0; i < nwords; i++) {
		w = *(wp++);
		rshift = 1;
		for (j = 0; j < LOGWLEN; j++) {
			w = (w & bitmask[j]) + ((w & ~bitmask[j]) >> rshift);
			rshift <<= 1;
		}
		count += w;
	}
	return (count);
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

