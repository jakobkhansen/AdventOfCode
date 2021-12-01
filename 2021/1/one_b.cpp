#include <iostream>

using namespace std;

int main() {
  int first, second, third, curr, incs = 0;

  cin >> first;

  cin >> second;
  cin >> third;

  while (cin >> curr) {
    if (curr > first) incs++;

    first = second;
    second = third;
    third = curr;
  }

  cout << incs << endl;
}
