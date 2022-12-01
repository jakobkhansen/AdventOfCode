#include <bits/stdc++.h>

using namespace std;

int main() {
  vector<int> crabs;

  int crab;

  while (scanf("%d,", &crab) != EOF) {
    crabs.push_back(crab);
  }
  int sum = 0;

  sum = accumulate(crabs.begin(), crabs.end(), sum);
  int average = (sum + crabs.size() - 1) / crabs.size();
  //int average = sum / crabs.size();

  int ops = 0;
  for (int i : crabs) {
    int diff = abs(i - average);
    int fuel = (diff+1)*(diff)/2;
    //cout << i << " " << fuel << endl;
    //cout << ((diff+1)*(diff))/2 << endl;
    ops += fuel;
  }
  cout << ops << endl;
}

