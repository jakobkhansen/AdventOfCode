#include <bits/stdc++.h>

using namespace std;

int main() {
  const int size = 12;
  int sums[size] = {};
  vector<string> oxygen;
  vector<string> lifesupp;

  string line;

  while (cin >> line) {
    oxygen.push_back(line);
    lifesupp.push_back(line);
  }

  for (int i = 0; i < size; i++) {
    int most_common = 0;

    for (string bitstring : oxygen) {
      most_common += bitstring[i] == '1' ? 1 : -1;
    }

    char correct = most_common >= 0 ? '1' : '0';
    vector<string> new_oxy;

    for (string bitstring : oxygen) {
      if (bitstring[i] == correct) new_oxy.push_back(bitstring);
    }

    oxygen = new_oxy;

    if (new_oxy.size() == 1) {
      break;
    }

  }

  for (int i = 0; i < size; i++) {
    int most_common = 0;

    for (string bitstring : lifesupp) {
      most_common += bitstring[i] == '1' ? 1 : -1;
    }

    char correct = most_common < 0 ? '1' : '0';
    vector<string> new_lifesupp;

    for (string bitstring : lifesupp) {
      if (bitstring[i] == correct) new_lifesupp.push_back(bitstring);
    }
    
    lifesupp = new_lifesupp;

    if (new_lifesupp.size() == 1) {
      break;
    }
  }

  bitset<size> oxy_final(oxygen[0]);
  bitset<size> lifesupp_final(lifesupp[0]);

  //cout << oxy_final << endl;
  //cout << lifesupp_final << endl;

  cout << oxy_final.to_ulong()*lifesupp_final.to_ulong() << endl;
}
