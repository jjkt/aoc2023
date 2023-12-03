#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include "file.hpp"
#include "stringutil.hpp"

#include <cstdint>
#include <iomanip> // std::setw, std::setfill
#include <cassert>

// input looks like this: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
// we output a vector of uint32_t, where each uint32_t is a set of 3 rgb values
auto parse_rgb_sets(std::string line) -> std::vector<uint32_t>
{
    std::vector<uint32_t> rgb_sets;

    // skip the "Game 1: " part:
    auto pos = line.find(":");
    if (pos == std::string::npos)
    {
        std::cout << "No ':' found in line: " << line << std::endl;
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
                rgb_set |= number << 16;
            }
            else if (color == "green")
            {
                rgb_set |= number << 8;
            }
            else if (color == "blue")
            {
                rgb_set |= number;
            }
            else
            {
                std::cout << "Unknown color: " << color << std::endl;
            }
        }
        rgb_sets.push_back(rgb_set);
    }

    return rgb_sets;
}

auto possible_game(int red, int green, int blue) -> bool
{
    return (red <= 12) && (green <= 13) && (blue <= 14);
}
auto main() -> int
{
    std::cout << "Advent of Code 2020 - Day 3" << std::endl;
    auto input = load_file("input.txt");

    std::cout << "Input size: " << input.size() << std::endl;

    int64_t sum_of_ok_indexes = 0;
    auto sum_of_powers = 0;
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
            auto red = (*set & 0xFF0000) >> 16;
            auto green = (*set & 0x00FF00) >> 8;
            auto blue = (*set & 0x0000FF);

            if (red > max_red)
            {
                max_red = red;
            }
            if (green > max_green)
            {
                max_green = green;
            }
            if (blue > max_blue)
            {
                max_blue = blue;
            }
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
            std::cout << " - POSSIBLE" << std::endl;
            sum_of_ok_indexes += game;
        }
        else
        {
            std::cout << " - IMPOSSIBLE" << std::endl;
        }
    }
    // print sum of ok indexes:
    std::cout << std::dec << "Sum of OK indexes: " << sum_of_ok_indexes << std::endl;
    std::cout << std::dec << "Sum of powers: " << sum_of_powers << std::endl;
}