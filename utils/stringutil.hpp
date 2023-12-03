#pragma once

#include <string>
#include <vector>

auto trim(std::string line) -> std::string
{
    auto pos = line.find_first_not_of(" ");
    if (pos == std::string::npos)
    {
        return "";
    }
    line = line.substr(pos);

    pos = line.find_last_not_of(" ");
    if (pos == std::string::npos)
    {
        return "";
    }
    line = line.substr(0, pos + 1);

    return line;
}

auto split(std::string line, std::string delimiter) -> std::vector<std::string>
{
    std::vector<std::string> parts;

    auto pos = line.find(delimiter);
    while (pos != std::string::npos)
    {
        auto part = line.substr(0, pos);
        parts.push_back(part);
        line = line.substr(pos + 1);
        pos = line.find(delimiter);
    }
    parts.push_back(line);

    return parts;
}

bool safe_isdigit(char ch)
{
    return std::isdigit(static_cast<unsigned char>(ch));
}

auto find_next_number(std::string line, size_t pos, size_t *len) -> size_t
{
    if (pos >= line.size())
    {
        return std::string::npos;
    }

    // find the first digit:
    while (pos < line.size() && !safe_isdigit(line[pos]))
    {
        ++pos;
    }

    if (pos == line.size())
    {
        return std::string::npos;
    }

    // find the last digit:
    size_t end = pos;
    while (end < line.size() && safe_isdigit(line[end]))
    {
        ++end;
    }

    *len = end - pos;

    return pos;
}