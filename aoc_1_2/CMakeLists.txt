cmake_minimum_required(VERSION 3.10)
project(aoc_1_2)
add_executable(aoc_1_2 aoc_1_2.cpp ../utils/file.cpp)
target_include_directories(aoc_1_2 PUBLIC ../utils)