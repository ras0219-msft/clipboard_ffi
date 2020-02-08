cmake_minimum_required(VERSION 3.10)
project(clipboard_rs CXX)

add_custom_target(
    cargo
    COMMAND cargo-fmt
    COMMAND set \"CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}/rust\"
    COMMAND cargo build
    USES_TERMINAL
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    BYPRODUCTS rust/debug/clipboard_rs.lib
)
add_library(clipboard_rs STATIC IMPORTED)
set_target_properties(clipboard_rs PROPERTIES
    IMPORTED_LOCATION "${CMAKE_CURRENT_BINARY_DIR}/rust/debug/clipboard_rs.lib"
    INTERFACE_LINK_LIBRARIES "Ws2_32.lib;Userenv.lib"
)

add_executable(clipboard main.cpp)
target_link_libraries(clipboard PRIVATE clipboard_rs)

add_custom_target(
    foo
    ALL
    COMMAND set RUST_BACKTRACE=1
    COMMAND $<TARGET_FILE:clipboard> paste -file foo.txt
)
