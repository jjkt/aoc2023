#include <iostream>
#include <string>
#include <vector>
#include "file.hpp"
#include "stringutil.hpp"

#include <cstdint>
#include <algorithm>

enum
{
    RED_MASK = 0xFF0000,
    GREEN_MASK = 0x00FF00,
    BLUE_MASK = 0x0000FF
};

enum
{
    RED_SHIFT = 16,
    GREEN_SHIFT = 8
};

// input looks like this: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
// we output a vector of uint32_t, where each uint32_t is a set of 3 rgb values
auto parse_rgb_sets(std::string line) -> std::vector<uint32_t>
{
    std::vector<uint32_t> rgb_sets;

    // skip the "Game 1: " part:
    auto pos = line.find(':');
    if (pos == std::string::npos)
    {
        std::cout << "No ':' found in line: " << line << '\n';
        return rgb_sets;
    }
    line = line.substr(pos + 1);

    // split with ";"
    auto sets = split(line, ";");

    // iterate sets:
    for (auto i = sets.begin(); i != sets.end(); ++i)
    {
        // split with ","
        auto rgb = split(*i, ",");

        // x red or y green or z blue

        // iterate rgb:
        uint32_t rgb_set = 0;
        for (auto j = rgb.begin(); j != rgb.end(); ++j)
        {
            // trim the string:
            *j = trim(*j);
            // split with " "
            auto number_and_color = split(*j, " ");

            // take the first number:
            auto number = std::stoi(number_and_color[0]);

            // take the color code:
            auto color = number_and_color[1];

            if (color == "red")
            {
                rgb_set |= number << RED_SHIFT;
            }
            else if (color == "green")
            {
                rgb_set |= number << GREEN_SHIFT;
            }
            else if (color == "blue")
            {
                rgb_set |= number;
            }
            else
            {
                std::cout << "Unknown color: " << color << '\n';
            }
        }
        rgb_sets.push_back(rgb_set);
    }

    return rgb_sets;
}

auto possible_game(unsigned int red, unsigned int green, unsigned int blue) -> bool
{
    return (red <= 12) && (green <= 13) && (blue <= 14); // NOLINT
}
auto main() -> int
{
    std::cout << "Advent of Code 2020 - Day 2" << '\n';
    auto input = load_file("input.txt");

    std::cout << "Input size: " << input.size() << '\n';

    int64_t sum_of_ok_indexes = 0;
    unsigned long sum_of_powers = 0;
    // iterate input lines:
    for (auto i = input.begin(); i != input.end(); ++i)
    {
        // find the leftmost number word:
        auto rgb_sets = parse_rgb_sets(*i);
        auto game = i - input.begin() + 1;

        std::cout << std::dec << "Game " << game << ": ";

        auto red_count = 0;
        auto green_count = 0;
        auto blue_count = 0;
        // print the set in hexadecimal and comma separated, 6 characters wide:
        auto possible = true;

        unsigned max_red = 0;
        unsigned max_green = 0;
        unsigned max_blue = 0;

        for (auto set = rgb_sets.begin(); set != rgb_sets.end(); ++set)
        {
            auto red = (*set & RED_MASK) >> RED_SHIFT;
            auto green = (*set & GREEN_MASK) >> GREEN_SHIFT;
            auto blue = (*set & BLUE_MASK);

            max_red = std::max(max_red, red);
            max_green = std::max(max_green, green);
            max_blue = std::max(max_blue, blue);

            std::cout << red << " red, " << green << " green, " << blue << " blue";
            if (set != rgb_sets.end() - 1)
            {
                std::cout << "; ";
            }

            // check that red would not overflow:
            if (!possible_game(red, green, blue))
            {
                possible = false;
            }
        }

        // print mins
        auto power = max_red * max_green * max_blue;
        sum_of_powers += power;
        std::cout << " - power: " << power;

        if (possible)
        {
            std::cout << " - POSSIBLE" << '\n';
            sum_of_ok_indexes += game;
        }
        else
        {
            std::cout << " - IMPOSSIBLE" << '\n';
        }
    }
    // print sum of ok indexes:
    std::cout << std::dec << "Sum of OK indexes: " << sum_of_ok_indexes << '\n';
    std::cout << std::dec << "Sum of powers: " << sum_of_powers << '\n';
}