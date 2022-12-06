# Did this quickly in Python because I only had 15 minutes
import sys
res = 0;
for line in sys.stdin:
    line = line.strip()
    first = [line[x] for x in range(len(line)//2)]
    second = [line[x] for x in range(len(line)//2, len(line))]
    for char in first:
        if char in second:
            if char.isupper():
                res += ord(char) - ord('A') + 27
            else:
                res += ord(char) - ord('a') + 1
            break

print(res)
