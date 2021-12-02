#include <bits/stdc++.h>

using namespace std;

int main() {
  string line, command;
  int commandVal = 0;
  int depth = 0;
  int hor = 0;

  while (getline(cin, line)) {
    stringstream ss(line);

    getline(ss, command, ' ');
    ss >> commandVal;

    if (command == "forward") hor += commandVal;
    if (command == "up") depth -= commandVal;
    if (command == "down") depth += commandVal;
  }
  cout << hor << endl;
  cout << depth << endl;

  cout << (hor*depth) << endl;
}
