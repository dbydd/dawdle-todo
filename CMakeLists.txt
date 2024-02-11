cmake_minimum_required(VERSION 3.28)
project(dawdle_todo)

set(CMAKE_CXX_STANDARD 20)

include(FetchContent)
FetchContent_Declare(
        Slint
        GIT_REPOSITORY https://github.com/slint-ui/slint.git
        GIT_TAG release/1
        SOURCE_SUBDIR api/cpp
)
FetchContent_MakeAvailable(Slint)

set(SRC src/main.cpp)
add_executable(dawdle_todo ${SRC})

set(UI ui/main_window.slint
    ui/title_bar.slint
    ui/todo_group.slint)
slint_target_sources(dawdle_todo ${UI})

target_link_libraries(dawdle_todo PRIVATE Slint::Slint)
if (WIN32)
    add_custom_command(TARGET dawdle_todo POST_BUILD COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_RUNTIME_DLLS:dawdle_todo> $<TARGET_FILE_DIR:dawdle_todo> COMMAND_EXPAND_LISTS)
endif ()