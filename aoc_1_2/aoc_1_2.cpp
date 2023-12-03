#include <iostream>
#include <string>
#include <vector>
#include "file.hpp"

const std::vector<std::string> numbers = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

auto find_leftmost_number_in_line(const std::string &line) -> int
{
    auto leftmost_position = std::string::npos;
    auto leftmost_number = -1;

    // leftmost is the smallest find() position. Need to test all to know for sure:
    for (auto i = 0; i < numbers.size(); ++i)
    {
        auto position = line.find(numbers[i]);
        if (position < leftmost_position)
        {
            leftmost_position = position;
            leftmost_number = i + 1;
        }
    }

    auto leftmost_digit = line.find_first_of("0123456789");

    if (leftmost_digit < leftmost_position)
    {
        leftmost_number = std::stoi(line.substr(leftmost_digit, 1));
    }

    return leftmost_number;
}

auto find_rightmost_number_in_line(const std::string &line) -> int
{
    auto rightmost_position = std::string::npos;
    auto rightmost_number = -1;

    // rightmost is the smallest rfind() position. Need to test all to know for sure:
    for (auto i = 0; i < numbers.size(); ++i)
    {
        auto position = line.rfind(numbers[i]);

        if (position == std::string::npos)
        {
            continue;
        }
        if (position > rightmost_position || rightmost_position == std::string::npos)
        {
            rightmost_position = position;
            rightmost_number = i + 1;
        }
    }

    // we need to check also the digits
    auto rightmost_digit = line.find_last_of("0123456789");

    if ((rightmost_digit > rightmost_position || rightmost_position == std::string::npos) && rightmost_digit != std::string::npos)
    {
        rightmost_number = std::stoi(line.substr(rightmost_digit, 1));
    }

    return rightmost_number;
}

auto main() -> int
{
    std::cout << "Advent of Code 2020 - Day 1 - part 2" << '\n';
    auto input = load_file("input.txt");

    std::cout << "Input size: " << input.size() << '\n';

    auto sum = 0;
    // iterate input lines:
    for (auto i = input.begin(); i != input.end(); ++i)
    {
        // find the leftmost number word:
        auto leftmost = find_leftmost_number_in_line(*i);
        auto rightmost = find_rightmost_number_in_line(*i);

        if (leftmost == -1 || rightmost == -1)
        {
            std::cout << "No integers found in line: " << *i << '\n';
            continue;
        }

        // take the leftmost and rightmost and combine to two digit number:
        auto number = leftmost * 10 + rightmost; // NOLINT
        sum += number;
    }
    std::cout << "Sum: " << sum << '\n';
}