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

@memoize
@listify
def fine(n, l, s):
    # sum <= s
    # len == l
    # max = n
    if l == 0:
        yield ''
    else:
        for i in xrange(n+1):
            if i <= s:
                for x in fine(n, l - 1, s - i):
                    yield str(i) + x

@memoize
def decay(v):
    n = len(v)
    cts = Counter(v)
    return ''.join(str(cts[str(i)]) for i in xrange(1, n+1))
from math import factorial as fac

def predc(v):
    n = len(v)
    v = map(int, v)
    if sum(v) > n: return 0
    v = [n - sum(v)] + v
    assert sum(v) == n
    res = fac(n)
    for x in v:
        res /= fac(x)
    return res

@memoize
def adjs(n):
    adj = defaultdict(list)
    for f in fine(n, n, n):
        adj[decay(f)].append(f)
    return adj

for cas in xrange(1,1+input()):
    s = line()
    ans = 0
    n = len(s)
    adj = adjs(n)
    res = [s]
    vis = {s}
    i = 0
    while i < len(res):
        t = res[i]; i += 1
        ans += predc(t) - len(adj[t])
        for u in adj[t]:
            if u not in vis:
                vis.add(u)
                res.append(u)
    ans += len(res)    

    #print "Case #%s:" % cas
    print "Case #%s: %s" % (cas, ans)
