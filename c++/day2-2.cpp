#include <cstdint>
#include <fstream>
#include <iostream>
#include <string.h>

uint16_t determineScore(std::string round) {
  uint16_t score = 0;
  uint16_t my_result = round[2] - 88;
  uint16_t their_move = round[0] - 64;
  if (my_result == 0) {
    switch (their_move) {
    case 1:
      score = 3;
      break;
    case 2:
      score = 1;
      break;
    case 3:
      score = 2;
      break;
    default:
      printf("Unexpected move\n");
      score = -1;
    }
  }
  if (my_result == 1) {
    score = their_move + 3;
  }
  if (my_result == 2) {
    switch (their_move) {
    case 1:
      score = 2 + 6;
      break;
    case 2:
      score = 3 + 6;
      break;
    case 3:
      score = 1 + 6;
      break;
    default:
      printf("Unexpected move\n");
      score = -1;
    }
  }
  if (score > 10) {
    printf("Faulty score = %i\nTheir move: %i\nResult: %i\n", score, their_move,
           my_result);
  } else {
    // printf("Score = %i\n", score);
  }
  return score;
}

int main(int argc, char const *argv[]) {
  std::string line;
  std::ifstream File("../inputs/day2.txt");
  uint16_t score;
  while (std::getline(File, line)) {
    score += determineScore(line);
  }
  std::cout << "Score is: " << score << "\n";
  return 0;
}
