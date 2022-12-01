#include <bits/stdc++.h>

using namespace std;

int main() {
  vector<int> crabs;

  int crab;

  while (scanf("%d,", &crab) != EOF) {
    crabs.push_back(crab);
  }
  int middle = crabs.size()/2;
  nth_element(crabs.begin(), crabs.begin() + middle, crabs.end());

  int median = crabs[middle];

  int ops = 0;
  for (int i : crabs) {
    ops += abs(i - median);
  }
  cout << ops << endl;
}
