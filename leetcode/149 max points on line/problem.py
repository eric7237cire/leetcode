# Definition for a point.
class Point:
     def __init__(self, a=0, b=0):
         self.x = a
         self.y = b

# From https://codereview.stackexchange.com/questions/66450/simplify-a-fraction
def gcd(a, b):
    """Calculate the Greatest Common Divisor of a and b.

        Unless b==0, the result will have the same sign as b (so that when
        b is divided by it, the result comes out positive).
        """
    while b:
        a, b = b, a % b
    return a

def simplify_fraction(numer, denom):

    if denom == 0:
        return "Division by 0 - result undefined"

    # Remove greatest common divisor:
    common_divisor = gcd(numer, denom)
    (reduced_num, reduced_den) = (numer / common_divisor, denom / common_divisor)
    # Note that reduced_den > 0 as documented in the gcd function.

    if reduced_den == 1:
        return (reduced_num, reduced_den)
        #return "%d/%d is simplified to %d" % (numer, denom, reduced_num)
    elif common_divisor == 1:
        return (reduced_num, reduced_den)
        #return "%d/%d is already at its most simplified state" % (numer, denom)
    else:
        return (reduced_num, reduced_den)
        #return "%d/%d is simplified to %d/%d" % (numer, denom, reduced_num, reduced_den)

class Solution:
    def maxPoints(self, points):
        """
        :type points: List[Point]
        :rtype: int
        """

        ybDict = {}

        n_Points = len(points)

        if n_Points <= 2 :
            return n_Points

        for i in range(0, n_Points):
            p1 = points[i]
            for j in range(i+1, n_Points):
                p2 = points[j]
                yDiff = p2.y - p1.y
                xDiff = p2.x - p1.x

                if xDiff == 0:
                    slope = None
                    slop_fraction = None
                    inter = p1.x
                else:
                    slop_fraction = simplify_fraction(yDiff, xDiff)
                    slope = yDiff / xDiff
                    inter = simplify_fraction( p2.y * slop_fraction[1] - slop_fraction[0] * p2.x, slop_fraction[1]) #                  ) p2.y - slope * p2.x
                    #print(f"Slope = {slope} {slop_fraction} intercep = {inter} for {p1} and {p2}")
                if (slop_fraction, inter) not in ybDict:
                    ybDict[(slop_fraction, inter)] = set()

                ybDict[(slop_fraction, inter)].add(i)
                ybDict[(slop_fraction, inter)].add(j)

        return max(len(s) for s in ybDict.values())





s = Solution()

checks = [

([[1, 1], [18, 19]], 2),
([[1, 1]], 1),
([], 0),
([[1, 1], [2, 2], [3, 3], [8,19], [9,19], [10,19], [11,19] ], 4),
   ([[1, 1], [2, 2], [3, 3]], 3),
    ([[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]], 4),
    ([[4,0],[4,-1],[4,5]], 3),
    ( [[0,-12],[5,2],[2,5],[0,-5],[1,5],[2,-2],[5,-4],[3,4],[-2,4],[-1,4],[0,-5],[0,-8],[-2,-1],[0,-11],[0,-9]], 6 ),

    ([[560,248],[0,16],[30,250],[950,187],[630,277],[950,187],[-212,-268],[-287,-222],[53,37],[-280,-100],[-1,-14],[-5,4],[-35,-387],
      [-95,11],[-70,-13],[-700,-274],[-95,11],[-2,-33],[3,62],[-4,-47],[106,98],[-7,-65],[-8,-71],[-8,-147],[5,5],[-5,-90],
      [-420,-158],[-420,-158],[-350,-129],[-475,-53],[-4,-47],[-380,-37],[0,-24],[35,299],[-8,-71],[-2,-6],[8,25],[6,13],[-106,-146],
      [53,37],[-7,-128],[-5,-1],[-318,-390],[-15,-191],[-665,-85],[318,342],[7,138],[-570,-69],[-9,-4],[0,-9],[1,-7],[-51,23],[4,1],[-7,5],
      [-280,-100],[700,306],[0,-23],[-7,-4],[-246,-184],[350,161],[-424,-512],[35,299],[0,-24],[-140,-42],[-760,-101],[-9,-9],[140,74],[-285,-21],
      [-350,-129],[-6,9],[-630,-245],[700,306],[1,-17],[0,16],[-70,-13],[1,24],[-328,-260],[-34,26],[7,-5],[-371,-451],[-570,-69],[0,27],[-7,-65],
      [-9,-166],[-475,-53],[-68,20],[210,103],[700,306],[7,-6],[-3,-52],[-106,-146],[560,248],[10,6],[6,119],[0,2],[-41,6],[7,19],[30,250]],
     22) ]

for tpl in checks:
    actual = s.maxPoints( [ Point(x[0], x[1]) for x in tpl[0] ])
    expected = tpl[1]

    if actual != expected:
        raise Exception(f"Error {actual} != {expected} for '{tpl[0]}'")
