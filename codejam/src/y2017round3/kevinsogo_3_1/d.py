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

mod = 10**9 + 7

INF = 10**100
for cas in xrange(1,1+input()):
    ans = 0

    def solve():
        r, c, n, d = parts()

        reqs = []
        for it in xrange(n):
            i, j, v = parts()
            i -= 1
            j -= 1
            reqs.append((i, j, v))
        for (i, j, v) in reqs:
            for (I, J, V) in reqs:
                if abs(v - V) > (abs(i - I) + abs(j - J))*d:
                    return 'IMPOSSIBLE'

        vis = {0, r-1}
        vjs = {0, c-1}
        for i, j, v in reqs:
            vis.add(i)
            vjs.add(j)
        vis = sorted(vis)
        vjs = sorted(vjs)
        grid = {}
        for I in vis:
            for J in vjs:
                grid[I, J] = INF
                for i, j, v in reqs:
                    grid[I, J] = min(grid[I, J], v + (abs(i - I) + abs(j - J)) * d)

        ans = 0

        def find(a, b, l):
            L = -1
            R = l+1
            # a + Ld <= b + (l - L)d
            # a + Rd > b + (l - R)d
            while R - L > 1:
                M = L +R  >> 1
                if a + M*d <= b + (l - M)*d:
                    L = M
                else:
                    R = M
            return L
        def line(a, b, l):
            L = find(a, b, l)
            ans = 0
            if 1 <= min(l,L+1):
                ans += a * (min(l,L+1) - 1)
                ans += d * min(l,L+1) * (min(l,L+1) - 1) / 2

            if max(1,L+1) <= l:
                rr = max(1,L+1)
                ans += (b + l*d)* (l - rr)
                ans -= (l*(l-1)/2 - rr*(rr-1)/2) * d
            return ans

        def loin(w, z, rr, ll, r ,c):
            w += ll * d
            n = r - ll + c
            nn = rr - ll + c
            b = rr - ll

            L = find(w, z, n)
            ans = 0
            lf = 1
            rg = min(nn, L+1, b+1, c)
            if lf <= rg:
                ans += w * (rg*(rg-1) - lf*(lf-1))/2
                ans += d * (rg*(rg-1)*(2*rg-1) - lf*(lf-1)*(2*lf-1))/6
            lf = max(1, c)
            rg = min(nn, L+1, b+1)
            if lf <= rg:
                ans += w * (c - 1) * (rg - lf)
                ans += d * (c - 1) * (rg*(rg-1) - lf*(lf-1))/2
            lf = max(1, b+1)
            rg = min(nn, L+1, c)
            if lf <= rg:
                ans += w * b * (rg - lf)
                ans += d * b * (rg*(rg-1) - lf*(lf-1))/2
            lf = max(1, b+1, c)
            rg = min(nn, L+1)
            if lf <= rg:
                ans += w * (b + c - 1) * (rg - lf)
                ans += d * (b + c - 1) * (rg*(rg-1) - lf*(lf-1))/2
                ans += w * (-1) * (rg*(rg-1) - lf*(lf-1))/2
                ans += d * (-1) * (rg*(rg-1)*(2*rg-1) - lf*(lf-1)*(2*lf-1))/6
            lf = max(1, L+1)
            rg = min(nn, b+1, c)
            if lf <= rg:
                ans += (z + n*d) * (rg*(rg-1) - lf*(lf-1))/2
                ans -= d * (rg*(rg-1)*(2*rg-1) - lf*(lf-1)*(2*lf-1))/6
            lf = max(1, L+1, c)
            rg = min(nn, b+1)
            if lf <= rg:
                ans += (z + n*d) * (c - 1) * (rg - lf)
                ans -= d * (c - 1) * (rg*(rg-1) - lf*(lf-1))/2
            lf = max(1, b+1, L+1)
            rg = min(nn, c)
            if lf <= rg:
                ans += (z + n*d) * b * (rg - lf)
                ans -= d * b * (rg*(rg-1) - lf*(lf-1))/2
            lf = max(1, L+1, b+1, c)
            rg = nn
            if lf <= rg:
                ans += (z + n*d) * (b + c - 1) * (rg - lf)
                ans -= d * (b + c - 1) * (rg*(rg-1) - lf*(lf-1))/2
                ans += (z + n*d) * (-1) * (rg*(rg-1) - lf*(lf-1))/2
                ans -= d * (-1) * (rg*(rg-1)*(2*rg-1) - lf*(lf-1)*(2*lf-1))/6
            return ans
        def box(w, x, y, z, r, c):

            ans = 0


            A = max(0, min(r, (y - w + r * d) / (2 * d)))
            B = max(0, min(r, (z + r * d - x) / (2 * d)))

            ll, rr = max(B+1, 1), min(A+1, r)
            if ll <= rr:
                ans += loin(w, z, rr, ll, r, c)

            ll, rr = max(A+1, 1), min(B+1, r)
            if ll <= rr:
                ans += loin(x, y, rr, ll, r, c)

            ll, rr = 1, min(A+1, B+1, r)
            if ll <= rr:
                ans += d * (c - 1) * (rr * (rr - 1) /2 - ll*(ll-1)/2)
                ans += line(w, x, c) * (rr - ll)
            ll, rr = max(A+1, B+1, 1), r
            if ll <= rr:
                ans -= d * (c - 1) * (rr * (rr - 1) /2 - ll*(ll-1)/2)
                ans += line(y + r*d, z + r*d, c) * (rr - ll)
            return ans
            # print >>stderr, 'hey'

            # for i in xrange(r+1):
            #     for j in xrange(c+1):
            #         print >>stderr, '.#'[grid[i][j] == w + (i + j) * d],
            #     print >>stderr
            # print >>stderr
            # for i in xrange(r+1):
            #     for j in xrange(c+1):
            #         print >>stderr, '.#'[grid[i][j] == x + (i + c - j) * d],
            #     print >>stderr
            # print >>stderr
            # for i in xrange(r+1):
            #     for j in xrange(c+1):
            #         print >>stderr, '.#'[grid[i][j] == y + (r - i + j) * d],
            #     print >>stderr
            # print >>stderr
            # for i in xrange(r+1):
            #     for j in xrange(c+1):
            #         print >>stderr, '.#'[grid[i][j] == z + (r + c - i - j) * d],
            #     print >>stderr
            # print >>stderr
            # assert grid[0][0] == w
            # assert grid[0][c] == x
            # assert grid[r][0] == y
            # assert grid[r][c] == z
            # return sum(sum(row[1:-1]) for row in grid[1:-1])


        for i in xrange(len(vis)):
            for j in xrange(len(vjs)):
                ans += grid[vis[i], vjs[j]]
                if j < len(vjs) - 1:
                    ans += line(grid[vis[i], vjs[j]], grid[vis[i], vjs[j + 1]], vjs[j + 1] - vjs[j])
                if i < len(vis) - 1:
                    ans += line(grid[vis[i], vjs[j]], grid[vis[i + 1], vjs[j]], vis[i + 1] - vis[i])
                if i < len(vis) - 1 and j < len(vjs) - 1:
                    ans += box(
                        grid[vis[i], vjs[j]],
                        grid[vis[i], vjs[j + 1]],
                        grid[vis[i + 1], vjs[j]],
                        grid[vis[i + 1], vjs[j + 1]],
                        vis[i + 1] - vis[i],
                        vjs[j + 1] - vjs[j],)



        return ans % mod
        # return sum(map(sum, grid)) % mod


    
    #print "Case #%s:" % cas
    print "Case #%s: %s" % (cas, solve())
