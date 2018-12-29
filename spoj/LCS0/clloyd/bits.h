
#define	WLEN	  32	/* word length */
#define	LOGWLEN	  5	/* log word length -- round(LOG2(WLEN) */
#define SMAX	  4096	/* maximum string length -- multiple of WLEN */
#define BITMAX	  128	/* maximum bit string length -- round(SMAX/WLEN) */
#define	ALPHASIZE 4	/* alphabet size */

typedef unsigned int WORD;
typedef short unsigned int INDEX;	/* i: alpha[i] == char */

extern char alpha[];	/* alphabet */

extern void randomstring();
extern void randomchange();
extern int lcs();
extern void alphastrings();
extern void bitops();
extern void simpleops();
extern int bitcount();
