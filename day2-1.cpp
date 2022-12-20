#include <cstdint>
#include <fstream>
#include <iostream>
#include <string.h>

uint16_t determineScore(std::string round) {
  uint16_t score = 0;
  uint8_t my_move = round[2] - 87;
  uint8_t their_move = round[0] - 64;

  if ((my_move == 1) && (their_move == 3)) {
    return 7;
  }
  if ((my_move == 3) && (their_move == 1)) {
    return 3;
  }
  if (my_move > their_move) {
    return my_move + 6;
  }
  if (my_move == their_move) {
    return my_move + 3;
  }
  if (my_move < their_move) {
    return my_move;
  }
  printf("Error\n");
  return -1;
}

int main(int argc, char const *argv[]) {
  std::string line;
  std::ifstream File("day2.txt");
  uint16_t score;

  while (std::getline(File, line)) {
    score += determineScore(line);
  }
  std::cout << "Score is: " << score << "\n";
  return 0;
}
