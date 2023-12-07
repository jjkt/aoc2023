#include <iostream>
#include <string>
#include <vector>
#include "file.hpp"

auto main() -> int
{
    std::cout << "Advent of Code 2020 - Day 1 - part 1" << '\n';
    auto input = load_file("input.txt");

    std::cout << "Input size: " << input.size() << '\n';

    auto sum = 0;
    // iterate input lines:
    for (auto i = input.begin(); i != input.end(); ++i)
    {
        auto leftmost_int = i->find_first_of("0123456789");
        auto rightmost_int = i->find_last_of("0123456789");
        if (leftmost_int == std::string::npos || rightmost_int == std::string::npos)
        {
            std::cout << "No integers found in line: " << *i << '\n';
            continue;
        }
        // take the leftmost and rightmost and combine to two digit number:
        auto leftmost = std::stoi(i->substr(leftmost_int, 1));
        auto rightmost = std::stoi(i->substr(rightmost_int, 1));
        auto number = leftmost * 10 + rightmost; // NOLINT
        sum += number;
    }
    std::cout << "Sum: " << sum << '\n';
}