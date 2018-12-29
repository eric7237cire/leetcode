class Solution:
    def strongPasswordChecker(self, s):
        """
        :type s: str
        :rtype: int
        """



# aaa 1 D or 1 R or 1 I
# aaa a 2 D, 1 R, 1 I
# aaa aa 3 D, 1 R, 1 I
# aaa aaa 4 D, 2 R, 2 I
# aaa aaa a 5 D, 2 R, 2 I
# aaa aaa aa 6 D  


s = Solution()

checks = {
"aaa": 3
}

for str in checks:
    actual = s.strongPasswordChecker(str)
    expected = checks[str]

    if actual != expected:
        raise Exception(f"Error {actual} != {expected} for '{str}'")