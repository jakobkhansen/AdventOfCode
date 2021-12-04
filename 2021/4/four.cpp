#include <bits/stdc++.h>

using namespace std;

const int boardSize = 5;

class Board {
  public:
    int board[boardSize][boardSize];
    bool won = false;

    bool hasBingo(unordered_map<int, bool> nums) {
      // Rows
      for (int i = 0; i < boardSize; i++) {
        bool bingo = true;
        for (int j = 0; j < boardSize; j++) {
          if (!nums[board[i][j]]) {
            bingo = false;
            break;
          }
        }
        if (bingo) {
          return true;
        }
      }

      // Columns
      for (int i = 0; i < boardSize; i++) {
        bool bingo = true;
        for (int j = 0; j < boardSize; j++) {
          if (!nums[board[j][i]]) {
            bingo = false;
            break;
          }
        }
        if (bingo) {
          return true;
        }
      }

      return false;
    }

    int getScore(unordered_map<int, bool> nums) {
      int sum = 0;

      for (int i = 0; i < boardSize; i++) {
        for (int j = 0; j < boardSize; j++) {
          if (!nums[board[i][j]]) {
            sum += board[i][j];
          }
        }
      }

      return sum;
    }

    string toString() {
      string out = "[ ";
      for (int i = 0; i < boardSize; i++) {
        out += "\n[ ";
        for (int j = 0; j < boardSize; j++) {
          out += to_string(board[i][j]) + " ";
        }
        out += "]";
      }
      return out + "\n]";
    }

};

vector<Board> createBoards() {

  vector<Board> boards;
  while (cin) {
    int tmp;
    Board newBoard;

    for (int i = 0; i < boardSize; i++) {
      for (int j = 0; j < boardSize; j++) {
        cin >> tmp;
        newBoard.board[i][j] = tmp;
      }
      cin.ignore();
    }

    cin.ignore();
    boards.push_back(newBoard);
  }
  // WHY
  boards.pop_back();

  return boards;

}

int calculateWinner(string selectionString, vector<Board> boards) {
  unordered_map<int, bool> nums;
  stringstream ss(selectionString);
  string numString;

  int numBoards = boards.size();

  while (getline(ss, numString, ',')) {
    int num = stoi(numString);
    nums[num] = true;

    for (Board &board : boards) {
      if (!board.won && board.hasBingo(nums)) {
        board.won = true;
        numBoards--;

        if (numBoards == 0) {
          int boardSum = board.getScore(nums);

          return boardSum*num;

        }

      }
    }
  }
  return 0;
}

int main() {

  string numbersString;
  cin >> numbersString;

  cin.ignore();


  vector<Board> boards = createBoards();

  cout << calculateWinner(numbersString, boards) << endl;

}
