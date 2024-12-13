import sys
def main():
    total = 0
    for line in sys.stdin:
        report = [int(x) for x in line.split()]

        for i in range(len(report)):
            print(report[:i], report[i+1:])
            new_report = report[:i] + report[i+1:]
            result = check_report(new_report)
            if result:
                total += 1
                break

    print(total)


def check_report(report):
    if report[0] == report[1]:
        return False

    increasing = report[0] < report[1]
    if not increasing: 
        report = list(reversed(report))
    for i,_ in enumerate(report[:-1]):
        increase = report[i+1] - report[i]
        if increase <= 0 or increase > 3:
            return False
    return True



main()
