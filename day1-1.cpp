#include <fstream>
#include <iostream>
#include <string.h>

int checkNewTop3(int current, int max) {
  if (current > max)
    return current;
  return max;
}

int main(int argc, char const *argv[]) {
  std::string line;
  std::ifstream File("day1.txt");

  int max = 0;
  int current = 0;
  while (std::getline(File, line)) {
    if (line.compare("") == 0) {
      max = checkNewTop3(current, max);
      current = 0;
      continue;
    }
    current += std::stoi(line);
  }
  std::cout << "Max value is: " << max << "\n";
  return 0;
}
