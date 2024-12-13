import re
import sys

inp = "".join([x for x in sys.stdin])
matches = re.findall("(?:mul\\(\\d+,\\d+\\))|(?:do\\(\\))|(?:don't\\(\\))", inp)

print(matches)
s = 0
enabled = True
for match in matches:
    if match == "do()":
        print("yes")
        enabled = True
    elif match == "don't()":
        print("no")
        enabled = False
    else:
        if not enabled:
            continue
        nums = re.findall("\\d+", match)
        s += int(nums[0]) * int(nums[1])
print(s)
