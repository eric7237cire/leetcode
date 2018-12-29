
#include <stdio.h>
#include "bits.h"
#include <sys/types.h>
#include <sys/times.h>

int seed;		/* for random */
/*** INDEX stringa[SMAX]; ***/	/* string 'a' -- index into alpha */
/*** INDEX stringb[SMAX]; ***/	/* string 'b' -- index into alpha */
INDEX stringa[SMAX] = {2,1,3,3,2,1,1,3,0,1,0,3,3,1,3,2};
INDEX stringb[SMAX] = {3,0,2,1,3,3,0,0,2,0,3,1,3,3,2,3};
char alpha[ALPHASIZE] = {'A', 'C', 'G', 'T'};	/* alphabet */

int stdinflag = 0;	/* input from standard input */
int checkflag = 0;	/* check bitlcs against simple lcs flag */

main(argc,argv)
int argc;
char **argv;
{
	int lena, lenb;
	register int i, count;
	register char **pname = argv;
	register int lbits, lsimple;
	register int tbits, tsimple, tinit;

	while (*argv[1] == '-') {
		if (argv[1][1] == 'c')
			checkflag++;
		else
			stdinflag++;
		argv++; argc--;
	}
	if (!((argc == 5) || (argc == 1))) {
		fprintf(stderr,
			"Usage: %s [-c] [-] [seed lena lenb count]\n", *pname);
		exit(1);
	}
	if (argc == 5) {
		seed = atoi(*(++argv));
		lena = atoi(*(++argv));
		lenb = atoi(*(++argv));
		count = atoi(*(++argv));
		printf("\n%d x %d\n", lena, lenb);
		printf("seed: %d, lena: %d, lenb: %d, count: %d\n",
			seed, lena, lenb, count);
		if (lena >= SMAX) {
			fprintf(stderr,"lena too large\n");
			exit(1);
		}
		if (lenb >= SMAX) {
			fprintf(stderr,"lenb too large\n");
			exit(1);
		}
		randomstring(stringa, lena);
		randomstring(stringb, lenb);
	} else {
		if (stdinflag) {
			fprintf(stderr,
				"Input from stdin not yet implemented\n");
			exit(1);
		} else {
			seed = 20;
			lena = 16;
			lenb = 16;
			count = 100;
			printf("\n%d x %d\n", lena, lenb);
			printf("seed: %d, lena: %d, lenb: %d, count: %d\n",
				seed, lena, lenb, count);
		}
	}
/***
	printf("string 'a': ");
	for (i = lena - 1; i >= 0; i--)
		if (i%WLEN == 0)
			printf("%c ", alpha[stringa[i]]);
		else
			printf("%c", alpha[stringa[i]]);
	printf("\n");
	printf("string 'b': ");
	for (i = lenb - 1; i >= 0; i--)
		if (i%WLEN ==0)
			printf("%c ", alpha[stringb[i]]);
		else
			printf("%c", alpha[stringb[i]]);
	printf("\n");
***/

	tinit = timing();
	for (i = 0; i < count; i++)
		lbits = bitlcs(stringa, lena, stringb, lenb);
	tbits = timing();
/*** fprintf(stderr, "Bits timing done\n"); ***/
	for (i = 0; i < count; i++)
		lsimple = simplelcs(stringa, lena, stringb, lenb);
	tsimple = timing();
/*** fprintf(stderr, "Simple timing done\n"); ***/
/***	printf("Start time %dmsec\n", tinit * 20);
	printf("End bits time %dmsec\n", tbits * 20);
	printf("End simple time %dmsec\n", tsimple * 20); ***/
	tsimple = tsimple - tbits;
	tbits = tbits - tinit;
	printf("Bit-lcs:    %d, total time: %dmsec, time: %dmsec, %%length(ave): %d%%\n",
		lbits, tbits*20, tbits*20/count, 200*lbits/(lena+lenb));
	printf("Simple-lcs: %d, total time: %dmsec, time: %dmsec, %%length(ave): %d%%\n",
		lsimple, tsimple*20, tsimple*20/count, 200*lsimple/(lena+lenb));
	if ((tbits == 0) || (tsimple == 0))
		printf("Times ratio: %d:%d (Bits:Simple)\n",
			tbits * 20 / tsimple * 20);
	else
		printf("Times ratio: %5.2f:1 (Simple:Bits)\n",
			(float) tsimple / (float) tbits);
	exit(0);
}

int random(n)
int n;
{
	/* Linear Congruential Pseudo Random Number Generator Knuth Vol2 */
	/* suitable for a Vax */

	register int r;

	seed = (seed * (1 + 4 * 37 * 109) + 9999) % (32 * 1024);
	r = (int) (((float) seed / (float) (32 * 1024)) * (float) n);
	return(r);
}

void randomstring(s, len)
INDEX *s;
int len;
{
	while (len--)
		*(s++) = random(ALPHASIZE);
	return;
}

static struct tms tms;

int timing()
{
	times(&tms);
/*** fprintf( stderr, "Current time: %din 1/50th sec\n", tms.tms_utime); ***/
	return ( (int) tms.tms_utime);
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

