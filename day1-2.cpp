#include <array>
#include <fstream>
#include <iostream>
#include <string.h>

std::array<int, 3> checkNewTop3(int current, std::array<int, 3> top_3) {
  if (current > top_3[0]) {
    top_3[2] = top_3[1];
    top_3[1] = top_3[0];
    top_3[0] = current;
    return top_3;
  }
  if (current > top_3[1]) {
    top_3[2] = top_3[1];
    top_3[1] = current;
    return top_3;
  }
  if (current > top_3[2]) {
    top_3[2] = current;
  }
  return top_3;
}

int main(int argc, char const *argv[]) {
  std::string line;
  std::ifstream File("day1.txt");

  int current = 0;
  std::array<int, 3> top_3;

  while (std::getline(File, line)) {
    if (line.compare("") == 0) {
      top_3 = checkNewTop3(current, top_3);
      current = 0;
      continue;
    }
    current += std::stoi(line);
  }
  std::cout << "Max value is: " << top_3[0] + top_3[1] + top_3[2] << "\n";
  return 0;
}
