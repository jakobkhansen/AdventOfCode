#include <bits/stdc++.h>

using namespace std;

const size_t numIterations = 256;

string toString(size_t vec[], size_t n) {
  string out = "[ ";
  for (size_t i = 0; i < n; i++) {
    out += to_string(vec[i]) + " ";
  }
  return out + "]";
}

int main() {
  size_t* fishesAtNum = new size_t[9];

  int fishNum;
  while (scanf("%d,", &fishNum) != EOF) {
    fishesAtNum[fishNum]++;
  }

  for (size_t i = 0; i < numIterations; i++) {
    size_t numNew = fishesAtNum[0];
    for (size_t j = 1; j < 9; j++) {
      fishesAtNum[j-1] = fishesAtNum[j];
    }
    fishesAtNum[6] += numNew;
    fishesAtNum[8] = numNew;
  }
  size_t sum = 0;
  sum = accumulate(fishesAtNum, fishesAtNum+9, sum);
  cout << sum << endl;
}
