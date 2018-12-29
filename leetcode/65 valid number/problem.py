import re

valid_numbers  = [
    re.compile("""
    \s*     # Whitespace
    [+\-]?  # Optional sign
    (?:
        # Either 1 digit before decimal
        (?:\d+
        \.?
        \d*) |
        # Or  with at least 1 digit after decimal
        (?:\d*
        \.?
        \d+)
    )
    # no spaces
    (e # e +- digits
        [+\-]?
        \d+
    )?
    \s*
               """, re.VERBOSE)
]

class Solution:
    def isNumber(self, s):
        """
        :type s: str
        :rtype: bool
        """
        for regex in valid_numbers:
            if regex.fullmatch(s):
                return True

        return False

s = Solution()

checks = {
"0" : True,
" 0.1 " : True,
"abc" : False,
"1 a" : False,
"2e10" : True,
" -90e3   " : True,
" 1e" : False,
"e3" : False,
" 6e-1" : True,
" 99e2.5 " : False,
"53.5e93" : True,
" --6 " : False,
"-+3" : False,
"95a54e53" : False,
    ".1": True,
"96 e5": False
}

for str in checks:
    actual = s.isNumber(str)
    expected = checks[str]

    if actual != expected:
        raise Exception(f"Error {actual} != {expected} for '{str}'")