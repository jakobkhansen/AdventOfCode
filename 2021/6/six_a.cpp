#include <bits/stdc++.h>

using namespace std;

const int numIterations = 256;

string toString(vector<int> vec, int n) {
  string out = "[ ";
  for (int i = 0; i < n; i++) {
    out += to_string(vec[i]) + " ";
  }
  return out + "]";
}

int main() {
  vector<int> fishes;

  int fishNum;
  int n = 0;
  while (scanf("%d,", &fishNum) != EOF) {
    fishes.push_back(fishNum);
    n++;
  }

  int previouslyAdded = 0;
  int newlyAdded = 0;
  for (int i = 0; i < numIterations+1; i++) {
    cout << to_string(n) << endl;
    n += previouslyAdded;
    previouslyAdded = newlyAdded;
    newlyAdded = 0;

    cout << "Iteration " << i << endl;
    //cout << toString(fishes, n) << endl;
    for (int i = 0; i < n; i++) {
      if (fishes[i] == 0) {
        fishes[i] = 6;
      } else {
        fishes[i]--;
      }

      if (fishes[i] == 0) {
        fishes.push_back(8);
        newlyAdded++;
      }
    }
  }
  cout << n << endl;

  // IDEA: Kalkuler hvor mange fisker en fisk vil generere etter 256 dager

}
