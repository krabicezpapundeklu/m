cmake_minimum_required(VERSION 3.16)
project(m)

set(CMAKE_CXX_STANDARD 17)

set(Boost_USE_STATIC_LIBS ON)
set(Boost_USE_STATIC_RUNTIME ON)

find_package(Boost REQUIRED COMPONENTS date_time filesystem regex)
find_package(Threads REQUIRED)

add_executable(m main.cpp)

set_property(TARGET m PROPERTY MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

target_compile_definitions(m PRIVATE _WIN32_WINNT=0x0601)
target_link_libraries(m PRIVATE Boost::date_time Boost::filesystem Boost::regex Threads::Threads)
