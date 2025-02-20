cmake_minimum_required(VERSION 3.10)
project(otter-tag-navigator)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 14)

# FetchContent configuration
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/release-1.12.1.zip
)
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

# Create a library from TagNavigator source (excluding its main)
add_library(TagNavigatorLib src/TagNavigator.cpp)

# Create the test executable using only test file (gtest_main provides main)
add_executable(tests test/test.cpp)
target_link_libraries(tests gtest_main TagNavigatorLib)

# Integrate with CTest
include(GoogleTest)
gtest_discover_tests(tests)