#include <bits/stdc++.h>

using namespace std;

int main() {
  string line, command;
  int commandVal = 0;
  int aim = 0;
  int depth = 0;
  int hor = 0;

  while (getline(cin, line)) {
    stringstream ss(line);

    getline(ss, command, ' ');
    ss >> commandVal;

    if (command == "forward") {
      hor += commandVal;
      depth += aim*commandVal;
    }
    if (command == "up") aim -= commandVal;
    if (command == "down") aim += commandVal;
  }

  cout << (hor*depth) << endl;
}
