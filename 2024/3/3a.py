import re
import sys

inp = "".join([x for x in sys.stdin])
matches = re.findall("(mul\\(\\d+,\\d+\\))", inp)

s = 0
for match in matches:
    nums = re.findall("\\d+", match)
    s += int(nums[0]) * int(nums[1])
print(s)
