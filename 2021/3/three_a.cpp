#include <bits/stdc++.h>

using namespace std;

int main() {
  const int size = 12;
  int sums[size] = {};

  string line;

  while (cin >> line) {
    for (int i = 0; i < size; i++) {
      char c = line[i];
      if (c == '1') sums[i]++;
      else sums[i]--;
    }
  }

  bitset<size> gamma;
  bitset<size> epsilon;

  for (int i = 0; i < size; i++) {
    gamma[size-i-1] = (sums[i] >= 0) ? 1 : 0;
    epsilon[size-i-1] = (sums[i] < 0) ? 1 : 0;
  }
  int result = gamma.to_ulong()*epsilon.to_ulong();
  cout << result << endl;
}
