cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
project(aoc_3)
add_executable(aoc_3 aoc_3.cpp ../utils/file.cpp)
target_include_directories(aoc_3 PUBLIC ../utils)