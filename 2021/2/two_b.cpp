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
    cout << command << " " << commandVal << endl;
    cout << "Hor: " << hor << endl;
    cout << "Depth: " << depth << endl;
    cout << "Aim: " << aim << endl;
    cout << endl;
  }

  cout << (hor*depth) << endl;
}
