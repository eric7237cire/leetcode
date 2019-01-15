if 1:
    from sys import *
    from functools import *
    from collections import *
    from itertools import *
    from functools import *
    from heapq import *
    xr = xrange

    def print_args(*args, **kwargs):
        return ','.join(value for value in [','.join(map(repr, args)),','.join("%s=%s" % (k, repr(v)) for k, v in kwargs.items())] if value)

    def print_result(before=False, after=True):
        def fc(func):
            @wraps(func)
            def f(*args, **kwargs):
                if before:
                    print "%s(%s)"      % (func.__name__, print_args(*args, **kwargs))
                r = func(*args, **kwargs)
                if after:
                    print "%s(%s) = %s" % (func.__name__, print_args(*args, **kwargs), r)
                return r
            return f
        return fc

    def memoize(function):
        memo = {}
        @wraps(function)
        def f(*args):
            key = args
            if key not in memo:
                memo[key] = function(*args)
                if not (len(memo) & 32767):
                    print >>stderr, "memo", function.__name__, len(memo)
            return memo[key]
        f.memo = memo
        return f

    def line():
        return raw_input().strip()

    def parts(f=int):
        return map(f, line().split())

    def qparts(f=int):
        data = line().split()
        return data[0], map(f, data[1:])

    def apply_after(after):
        def _apply_after(function):
            @wraps(function)
            def f(*args, **kwargs):
                return after(function(*args, **kwargs))
            return f
        return _apply_after

    listify = apply_after(list)

INF = 10**100

for cas in xrange(1,1+input()):

    def solve():
        ans = 0
        n, e = parts()
        val = [0]*e
        edges = [None]*e
        adj = [[] for i in xrange(n)]
        for i in xrange(e):
            a, b = parts()
            a -= 1
            b -= 1
            edges[i] = a, b
            adj[a].append((b, i, 1))
            adj[b].append((a, i, -1))
        for s in xrange(e):
            if val[s] == 0:
                a, b = edges[s]
                # print 'hey', s, a, b
                sign = {}
                sign[s] = 1
                stack = [b]
                parent = {}
                parente = {}
                vis = {b}
                while stack:
                    i = stack.pop()
                    if i == a:
                        break
                    for j, x, sig in adj[i]:
                        if x != s and j not in vis:
                            vis.add(j)
                            parent[j] = i
                            parente[j] = x
                            sign[x] = sig
                            stack.append(j)
                    # for x, (A, B) in enumerate(edges):
                    #     if x != s and i == A:
                    #         if B not in vis:
                    #             vis.add(B)
                    #             parent[B] = i
                    #             parente[B] = x
                    #             sign[x] = 1
                    #             stack.append(B)
                    #     if x != s and i == B:
                    #         if A not in vis:
                    #             vis.add(A)
                    #             parent[A] = i
                    #             parente[A] = x
                    #             sign[x] = -1
                    #             stack.append(A)
                else:
                    return

                sign[s] = 1
                req = [s]
                while i != b:
                    req.append(parente[i])
                    i = parent[i]

                # print 'cyc', req

                mxa = 0
                for i in req:
                    mxa = max(mxa, abs(val[i]))
                # print 'how', mxa
                best = INF
                for v in xrange(-mxa-2, mxa+3):
                    bago = 0
                    for i in req:
                        bagu = abs(val[i] + sign[i] * v)
                        if bagu == 0:
                            bago = INF
                            break
                        bago = max(bago, bagu)
                    # if bago < INF: print 'hoy', v, bago
                    if best > bago:
                        best = bago
                        bestv = v


                v = bestv
                for i in req:
                    val[i] = val[i] + sign[i] * v

                for i in xrange(n):
                    total = 0
                    for x, (a, b) in enumerate(edges):
                        if i == a:
                            total += val[x]
                        if i == b:
                            total -= val[x]
                    assert total == 0
                    assert -n**2 <= total <= n**2

        return val

    
    res = solve()
    print "Case #%s:" % cas,
    if not res:
        print 'IMPOSSIBLE'
    else:
        print ' '.join(map(str, res))

