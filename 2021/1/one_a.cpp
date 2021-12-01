#include <iostream>

using namespace std;

int main() {
  int prev, curr;
  int increases = 0;


  cin >> prev;

  while (cin >> curr) {
    if (curr > prev) increases++;
    prev = curr;
  }
  cout << increases << endl;
}
