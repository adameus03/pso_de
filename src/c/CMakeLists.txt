cmake_minimum_required(VERSION 3.0.0)
project(differential_evolution_cmake VERSION 0.1.0 LANGUAGES C)

add_library(${PROJECT_NAME} de.c mtwister.c)

target_include_directories( ${PROJECT_NAME}
    PUBLIC ${PROJECT_SOURCE_DIR}/.
)