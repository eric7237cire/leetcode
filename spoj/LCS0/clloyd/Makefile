OBJECTS = lcsame.o bitlcs.o simplelcs.o bitops.o
SOURCES = lcsame.c bitlcs.c simplelcs.c bitops.s
OBJECTS1 = lcsptrs.o bitlcs.o simplelcs.o bitops.o
SOURCES1 = lcsptrs.c bitlcs.c simplelcs.c bitops.s

lcsame :	$(OBJECTS)
	cc -o lcsame $(OBJECTS)

$(SOURCES) :	bits.h

