import sys
def main():
    total = 0
    for line in sys.stdin:
        report = [int(x) for x in line.split()]

        if report[0] == report[1]:
            continue

        increasing = report[0] < report[1]
        if not increasing: 
            report = list(reversed(report))
        safe = 1
        for i,_ in enumerate(report[:-1]):
            increase = report[i+1] - report[i]
            if increase <= 0 or increase > 3:
                safe = 0
                break
        total += safe

    print(total)




main()
