#include <iostream>
#include <fstream>
#include <string>
#include <vector>


auto load_file(std::string filename) -> std::vector<std::string>
{
    std::ifstream input_file(filename);
    std::string line;
    std::vector<std::string> input;
    while (std::getline(input_file, line))
    {
        input.push_back(line);
    }
    return input;
}