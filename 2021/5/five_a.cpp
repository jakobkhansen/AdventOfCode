#include <bits/stdc++.h>

using namespace std;

inline size_t key(int i, int j) {
  return (size_t) i << 32 | (unsigned int) j;
}

bool isStraightLine(int x1, int y1, int x2, int y2) {
  return x1 == x2 || y1 == y2;
}

void addPositionsFromStraightLine(unordered_map<size_t, int>* map, int x1, int y1, int x2, int y2) {
  int xIncrement = x1 < x2 ? 1 : -1;
  while (x1 != x2) {
    (*map)[key(x1,y1)] = (*map)[key(x1,y1)] + 1;
    x1 += xIncrement;
  }

  int yIncrement = y1 < y2 ? 1 : -1;
  while (y1 != y2) {
    (*map)[key(x1,y1)] = (*map)[key(x1,y1)] + 1;
    y1 += yIncrement;
  }
  (*map)[key(x1,y1)] = (*map)[key(x1,y1)] + 1;
}

void addPositionsFromDiagonalLine(unordered_map<size_t, int>* map, int x1, int y1, int x2, int y2) {
  int xIncrement = x1 < x2 ? 1 : -1;
  int yIncrement = y1 < y2 ? 1 : -1;

  while (x1 != x2 and y1 != y2) {
    (*map)[key(x1,y1)] = (*map)[key(x1,y1)] + 1;
    x1 += xIncrement;
    y1 += yIncrement;
  }

  (*map)[key(x1,y1)] = (*map)[key(x1,y1)] + 1;
}

int main() {
  unordered_map<size_t, int> map;

  int x1, y1, x2, y2;

  while (scanf("%d,%d -> %d,%d\n", &x1, &y1, &x2, &y2) != EOF) {
    //cout << x1 << " " << y1 << " " << x2 << " " << y2 << endl;
    if (isStraightLine(x1, y1, x2, y2)) {
      addPositionsFromStraightLine(&map, x1, y1, x2, y2);
    } else {
      addPositionsFromDiagonalLine(&map, x1, y1, x2, y2);
    }
  }

  int numOverlapping = 0;
  for (auto& pair : map) {
    if (pair.second > 1) {
      numOverlapping++;
    }
  }
  cout << numOverlapping << endl;
}
