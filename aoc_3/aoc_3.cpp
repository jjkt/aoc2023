#include <iostream>
#include <string>
#include <vector>
#include "file.hpp"
#include "stringutil.hpp"

#include <cassert>
#include <cstddef>
#include <utility>
#include <ranges>

struct span_t
{
    size_t pos;
    size_t len;
};

struct number
{
    int value;
    span_t span;
};

[[nodiscard]] auto is_symbol(std::string line, size_t pos) -> bool
{
    assert(pos < line.size());
    return !safe_isdigit(line[pos]) && line[pos] != '.';
}

[[nodiscard]] auto touched_by_a_symbol_on_a_line(const std::string &line, span_t span) -> bool
{
    if (!line.empty())
    {
        int index = (int)span.pos - 1;
        const size_t chars_to_check = span.len + 2;
        for (int count = 0; count < (chars_to_check) && (index < (int)line.size()); ++index, ++count)
        {
            if (index < 0)
            {
                continue;
            }
            if (is_symbol(line, index))
            {
                return true;
            }
        }
    }
    return false;
}

[[nodiscard]] auto get_numbers(const std::string &line) -> std::vector<number>
{
    std::vector<number> numbers;
    size_t pos = 0;
    size_t len = 0;

    while ((pos = find_next_number(line, pos, &len)) != std::string::npos)
    {
        auto number = line.substr(pos, len);
        numbers.push_back({std::stoi(number), pos, len});
        pos += len;
    }
    return numbers;
}

[[nodiscard]] auto two_ranges_overlap(size_t start1, size_t end1, size_t start2, size_t end2) -> bool
{
    return (start1 <= end2) && (end1 >= start2);
}

[[nodiscard]] auto get_numbers_touching_range(const std::vector<number> &numbers, size_t start, size_t end) -> std::vector<number>
{
    // C++20 ranges version:
    auto result = numbers | std::ranges::views::filter([start, end](const number &number)
                                                       { return two_ranges_overlap(number.span.pos, number.span.pos + number.span.len - 1, start, end); });

    return std::vector<number>{result.begin(), result.end()};
}

[[nodiscard]] auto process(const std::string &previous_line, std::string middle_line, const std::string &next_line, unsigned *gear_ratio_sum) -> unsigned
{
    assert(middle_line.size() > 0);

    const std::vector<number> previous_line_numbers = get_numbers(previous_line);
    const std::vector<number> middle_line_numbers = get_numbers(middle_line);
    const std::vector<number> next_line_numbers = get_numbers(next_line);

    auto sum = 0;

    for (auto number : middle_line_numbers)
    {
        if (touched_by_a_symbol_on_a_line(previous_line, number.span))
        {
            sum += number.value;
            continue;
        }

        // Left and right from current line
        if (number.span.pos > 0 && is_symbol(middle_line, number.span.pos - 1))
        {
            sum += number.value;
            continue;
        }

        auto right_index = number.span.pos + number.span.len;
        if (right_index < middle_line.size() && is_symbol(middle_line, right_index))
        {
            sum += number.value;
            continue;
        }

        if (touched_by_a_symbol_on_a_line(next_line, number.span))
        {
            sum += number.value;
        }
    }

    // Part 2: check for gears. Gears are marked with '*'.
    for (auto i = 0; i < middle_line.size(); ++i)
    {
        if (middle_line[i] == '*')
        {
            // get the number of adjacent numbers:
            auto numbers_on_previous_line = get_numbers_touching_range(previous_line_numbers, i - 1, i + 1);
            auto numbers_on_left = get_numbers_touching_range(middle_line_numbers, i - 1, i - 1);
            auto numbers_on_right = get_numbers_touching_range(middle_line_numbers, i + 1, i + 1);
            auto numbers_on_next_line = get_numbers_touching_range(next_line_numbers, i - 1, i + 1);

            std::vector<number> adjacent_numbers;
            adjacent_numbers.reserve(numbers_on_previous_line.size() + numbers_on_left.size() + numbers_on_right.size() + numbers_on_next_line.size());
            adjacent_numbers.insert(adjacent_numbers.end(), numbers_on_previous_line.begin(), numbers_on_previous_line.end());
            adjacent_numbers.insert(adjacent_numbers.end(), numbers_on_left.begin(), numbers_on_left.end());
            adjacent_numbers.insert(adjacent_numbers.end(), numbers_on_right.begin(), numbers_on_right.end());
            adjacent_numbers.insert(adjacent_numbers.end(), numbers_on_next_line.begin(), numbers_on_next_line.end());

            if (adjacent_numbers.size() == 2)
            {
                // multiply adjacent numbers:
                auto gear_ratio = adjacent_numbers[0].value * adjacent_numbers[1].value;
                *gear_ratio_sum += gear_ratio;
            }
        }
    }

    return sum;
}

auto process_file(std::string filename)
{
    auto input = load_file(std::move(filename));

    std::cout << "Input size: " << input.size() << '\n';

    std::string previous_line;
    std::string middle_line;

    // iterate lines:
    unsigned long sum = 0;
    unsigned gear_ratio_sum = 0;
    for (auto i = input.begin(); i != input.end(); ++i)
    {
        if (!middle_line.empty())
        {
            sum += process(previous_line, middle_line, *i, &gear_ratio_sum);
        }
        previous_line = middle_line;
        middle_line = *i;
    }

    sum += process(previous_line, middle_line, "", &gear_ratio_sum);
    std::cout << "Sum: " << sum << '\n';
    std::cout << "Gear ratio sum: " << gear_ratio_sum << '\n';
}

auto main() -> int
{
    std::cout << "Advent of Code 2020 - Day 3" << '\n';

    process_file("input.txt");
    process_file("input_full.txt");
}