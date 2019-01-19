from __future__ import division, print_function
input = raw_input

t = int(input())

INF = 2 * 10 ** 18
MOD = 10 ** 9 + 7

def T(n):
    return n * (n - 1) // 2

def R(a,c1, c2,d):
    # return sum(min(c1,c2-d*i) for i in range(a))
    cc = max(0,min(a,(c2-c1+d-1)//d))
    s = 0
    s += c1 * cc
    s += c2 * (a - cc)
    s -= d * (T(a)-T(cc))
    return s

def sumT(n):
    return n * (n-1) * (n-2) // 6

def U(r,c,z):
    s = 0
    #for i in range(min(r,z)):
    #    s += min(c,z-i)
    s += R(min(r,z),c,z,1)
    return s

def Q(n):
    return n * (n - 1) * (2 * n - 1) // 6

def V(r,c,z):
    s = 0
    a = min(r,z)
    cc = max(0,min(a,z-c))
    #for i in range(cc):
    #    s += i * c + T(c)
    s += c * T(cc) + cc * T(c)
    s += z * (T(a) - T(cc))
    s -= Q(a) - Q(cc)
    #for i in range(z-a+1,z-cc+1):
    #    s += T(i)
    s += sumT(z-cc+1)-sumT(z-a+1)
    return s

def f(c1,c2,c3,c4,d,rr,cc):
    s = 0
    c2 += d*rr
    c3 += d*cc
    c4 += d*rr+d*cc
    cutoff1i = max(0,min(rr,(c2-c1+2*d-1)//(2*d)))
    cutoff1j = max(0,min(cc,(c3-c1+2*d-1)//(2*d)))
    cutoff2i = max(cutoff1i,min(rr,(c4-c3+2*d-1)//(2*d)))
    cutoff1ij = max(0,min(cutoff1i+cutoff1j,(c4-c1+2*d-1)//(2*d)))
    s += d * cc * (rr * (rr - 1)) // 2
    s += d * rr * (cc * (cc - 1)) // 2
    #i = cutoff2-1-i
    s += 2 * d * cc * T(cutoff2i-cutoff1i)
    #for i in range(cutoff2i-cutoff1i):
    #    for j in range(cc):
    #        s += min(c2-2*d*(cutoff2i-1),c3-2*d*j-2*d*i)
    y = c2-2*d*(cutoff2i-1)
    s -= 2*d*cc*T(cutoff2i-cutoff1i)
    s -= 2*d*(cutoff2i-cutoff1i)*T(cc)
    cutoff2ij = max(0,min(cutoff2i-cutoff1i+cc,(c3-y+2*d-1)//(2*d)))
    s += U(cutoff2i-cutoff1i,cc,cutoff2ij)*(y-c3)
    s += 2*d*V(cutoff2i-cutoff1i,cc,cutoff2ij)
    s += c3*(cutoff2i-cutoff1i)*cc
    s -= cc * 2 * d * (T(rr) - T(cutoff2i))
    #for i in range(cutoff2i,rr):
    #    s -= cc * 2 * d * i
    #    #for j in range(cc):
    #    #    s += min(c2,c4-2*d*j)
    s += (rr - cutoff2i) * R(cc,c2,c4,2*d)
    s -= 2 * d * cutoff1i * (T(cc) - T(cutoff1j))
    s += (cc-cutoff1j) * R(cutoff1i,c3,c4,2*d)
    s += c4 * cutoff1i * cutoff1j
    s -= cutoff1j * 2 * d * T(cutoff1i)
    s -= cutoff1i * 2 * d * T(cutoff1j)
    s += U(cutoff1i,cutoff1j,cutoff1ij) * (c1 - c4)
    s += V(cutoff1i,cutoff1j,cutoff1ij) * 2 * d
    #for i in range(cutoff1i):
    #    for j in range(cutoff1j):
    #        if i+j < cutoff1ij:
    #            s += c1-c4 + 2 * d * (i+j)
    #        #s += min(c1,c4-2*d*i-2*d*j)
    return s

for tc in range(1,t+1):
    print("Case #%d:"%tc, end = " ")
    r,c,n,d = map(int,input().split())
    l = []
    for _ in range(n):
        l.append(tuple(map(int,input().split())))
    rows = set()
    cols = set()
    rows.add(1)
    rows.add(r+1)
    cols.add(1)
    cols.add(c+1)
    for ri,ci,bi in l:
        rows.add(ri)
        cols.add(ci)
    rw = sorted(rows)
    cw = sorted(cols)
    rwd = {}
    cwd = {}
    for i in range(len(rw)):
        rwd[rw[i]] = i
    for i in range(len(cw)):
        cwd[cw[i]] = i
    impossible = False
    for i in range(len(rw)-1):
        for j in range(len(cw)-1):
            mn = 1
            mx = INF
            for (ri,ci,bi) in l:
                mn = max(mn,bi-d*(abs(ri-rw[i])+abs(ci-cw[j])))
                mx = min(mx,bi+d*(abs(ri-rw[i])+abs(ci-cw[j])))
            #print(rw[i],cw[j],mn,mx)
            if mn > mx:
                impossible = True
                break
        if impossible: break
    if impossible:
        print("IMPOSSIBLE")
        continue

    s = 0
    for i in range(len(rw)-1):
        for j in range(len(cw)-1):
            r1,c1 = rw[i],cw[j]
            r2,c2 = rw[i+1],cw[j+1]
            mx1,mx2,mx3,mx4 = INF,INF,INF,INF
            for (ri,ci,bi) in l:
                if ri <= r1 and ci <= c1:
                    mx1 = min(mx1,bi+d*((r1-ri)+(c1-ci)))
                elif ri <= r1 and ci >= c2:
                    mx3 = min(mx3,bi+d*((r1-ri)+(ci-c2)))
                elif ri >= r2 and ci <= c1:
                    mx2 = min(mx2,bi+d*((ri-r2)+(c1-ci)))
                elif ri >= r2 and ci >= c2:
                    mx4 = min(mx4,bi+d*((ri-r2)+(ci-c2)))
                else: assert False
            #print(mx1,mx2,mx3,mx4,r1,r2,c1,c2,d,f(mx1,mx2,mx3,mx4,d,r2-r1,c2-c1))
            s = (s + f(mx1,mx2,mx3,mx4,d,r2-r1,c2-c1)) % MOD
    print(s % MOD)
