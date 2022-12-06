
import sys

res = 0;
lines = list(sys.stdin)
for i in range(0, len(lines), 3):

    curr_lines = [lines[x].strip() for x in range(i, i+3)]
    for char in curr_lines[0]:
        if char in curr_lines[1] and char in curr_lines[2]:
            if char.isupper():
                res += ord(char) - ord('A') + 27
            else:
                res += ord(char) - ord('a') + 1
            break

print(res)
