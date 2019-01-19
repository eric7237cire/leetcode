from sys import stdin

def getInt():
    return int(stdin.readline())

def getInts():
    return tuple(int(z) for z in stdin.readline().split())

def fn3(a,b,c,k):
    return a*k+b*k*(k-1)/2+c*k*(k-1)*(2*k-1)/6

def fn2(a,b,c,k,l):
    if k >= l:
        return 0
    return fn3(a,b,c,l)-fn3(a,b,c,k)

def fn(cons, mx, my, ms, d):
    if ms < 0:
        return 0
    tot = fn2(2*cons*(ms+1)+d*ms+d*ms*ms,(2*d*ms+2*d-2*cons-d-2*d*ms),-d,max(0,(ms-my)+1),min(ms,mx)+1)/2
    if my >= 0:
        tot += fn2(cons*(my+1)+d*my*(my+1)/2,d*(my+1),0,0,min(mx,ms-my)+1)
    return tot
        

t = getInt()

for cn in xrange(1,1+t):
    (r,c,n,d) = getInts()
    rcbs = [getInts() for i in xrange(n)]
    
    if any(abs(b1-b2) > d * (abs(r1-r2)+abs(c1-c2)) for (r1,c1,b1) in rcbs for (r2,c2,b2) in rcbs):
        print "Case #{}: IMPOSSIBLE".format(cn)
    else:
        rs = sorted(list(set([1,r+1]+[z[0] for z in rcbs])))
        cs = sorted(list(set([1,c+1]+[z[1] for z in rcbs])))
        tot = 0
        for i in xrange(1,len(rs)):
            for j in xrange(1,len(cs)):
                (r0,r1) = (rs[i-1],rs[i])
                (c0,c1) = (cs[j-1],cs[j])
                (q0,q1,q2,q3) = (0,0,0,0)
                (e0,e1,e2,e3) = (False,False,False,False)
                for (rr,cc,b) in rcbs:
                    if rr <= r0 and cc <= c0:
                        if e0:
                            q0 = min(b-d*rr-d*cc,q0)
                        else:
                            q0 = b-d*rr-d*cc
                            e0 = True
                    elif rr >= r1 and cc <= c0:
                        if e1:
                            q1 = min(b+d*rr-d*cc,q1)
                        else:
                            q1 = b+d*rr-d*cc
                            e1 = True
                    elif rr <= r0 and cc >= c0:
                        if e2:
                            q2 = min(b-d*rr+d*cc,q2)
                        else:
                            q2 = b-d*rr+d*cc
                            e2 = True
                    elif rr >= r1 and cc >= c0:
                        if e3:
                            q3 = min(b+d*rr+d*cc,q3)
                        else:
                            q3 = b+d*rr+d*cc
                            e3 = True
                if e0:
                    minx = r0
                    maxx = min(r1-1,(q1-q0)/(2*d)) if e1 else r1-1
                    miny = c0
                    maxy = min(c1-1,(q2-q0)/(2*d)) if e2 else c1-1
                    mins = r0+c0
                    maxs = min(r1+c1-2,(q3-q0)/(2*d)) if e3 else r1+c1-2
                    if (minx <= maxx) and (miny <= maxy) and (mins <= maxs):
                        tot += fn(d*minx+d*miny+q0,maxx-minx,maxy-miny,maxs-(minx+miny),d)
                if e1:
                    minx = max((q1-q0)/(2*d)+1,r0) if e0 else r0
                    maxx = r1-1
                    mind = c0-(r1-1)
                    maxd = min((q2-q1)/(2*d),(c1-1)-r0) if e2 else (c1-1)-r0
                    miny = c0
                    maxy = min(c1-1,(q3-q1)/(2*d)) if e3 else c1-1
                    if (minx <= maxx) and (miny <= maxy) and (mind <= maxd):
                        tot += fn(-d*maxx+d*miny+q1,maxx-minx,maxy-miny,maxd+maxx-miny,d)
                if e2:
                    minx = r0
                    maxx = min(r1-1,(q3-q2)/(2*d)) if e3 else r1-1
                    mind = max((q2-q1)/(2*d)+1,c0-(r1-1)) if e1 else c0-(r1-1)
                    maxd = (c1-1)-r0
                    miny = max((q2-q0)/(2*d)+1,c0) if e0 else c0
                    maxy = c1-1
                    if (minx <= maxx) and (miny <= maxy) and (mind <= maxd):
                        tot += fn(-d*maxy+d*minx+q2,maxx-minx,maxy-miny,maxy-mind-minx,d)
                if e3:
                    mins = max((q3-q0)/(2*d)+1,r0+c0) if e0 else r0+c0
                    maxs = r1+c1-2
                    miny = max((q3-q1)/(2*d)+1,c0) if e1 else c0
                    maxy = c1-1
                    minx = max((q3-q2)/(2*d)+1,r0) if e2 else r0
                    maxx = r1-1
                    if (minx <= maxx) and (miny <= maxy) and (mins <= maxs):
                        tot += fn(-d*maxx-d*maxy+q3,maxx-minx,maxy-miny,(maxx+maxy)-mins,d)
        print "Case #{}: {}".format(cn, tot % 1000000007)
