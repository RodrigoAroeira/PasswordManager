
cmake_minimum_required(VERSION 3.10)

project(PasswordManager LANGUAGES CXX)

file(GLOB SRC_FILES "src/*.cpp")

# Ensure the local include path is always considered
include_directories(headers includes/usr/include)

# Target for dependencies that must be downloaded if not found
add_custom_target(deps
    COMMAND ${CMAKE_COMMAND} -E make_directory includes
    COMMAND bash -c "if [ ! -f includes/usr/include/sqlite3.h ]; then \
        apt-get download libsqlite3-dev && \
        dpkg-deb -xv libsqlite3-dev*.deb includes && \
        rm libsqlite3-dev*.deb; \
    fi"
    COMMAND bash -c "if [ ! -f includes/usr/include/openssl/ssl.h ]; then \
        apt-get download libssl-dev && \
        dpkg-deb -xv libssl-dev*.deb includes && \
        rm libssl-dev*.deb; \
    fi"
)

# Make sure dependencies are downloaded before continuing
add_dependencies(${PROJECT_NAME} deps)

# Run dependency check after downloading, with local include and library paths set up
find_package(PkgConfig REQUIRED)

# Try to find SQLite3 and OpenSSL using pkg-config, fallback to local include if not found
pkg_check_modules(SQLITE3 QUIET sqlite3)
pkg_check_modules(OPENSSL QUIET openssl)

if (NOT SQLITE3_FOUND)
    include_directories(includes/usr/include)
    link_directories(includes/usr/lib)
endif()

if (NOT OPENSSL_FOUND)
    include_directories(includes/usr/include)
    link_directories(includes/usr/lib)
endif()

# Generate an error if the dependencies were not properly found
if (NOT SQLITE3_FOUND OR NOT OPENSSL_FOUND)
    message(FATAL_ERROR "Could not find required packages: sqlite3, openssl")
endif()

# Create the main executable
add_executable(PasswordManager ${SRC_FILES})

target_link_libraries(PasswordManager ${SQLITE3_LIBRARIES} ${OPENSSL_LIBRARIES})

target_include_directories(PasswordManager PRIVATE ${SQLITE3_INCLUDE_DIRS} ${OPENSSL_INCLUDE_DIRS})

# Additional clean-up targets
add_custom_target(remove
  COMMAND ${CMAKE_COMMAND} -E remove -f PasswordManager
)

add_custom_target(veryclean
  COMMAND ${CMAKE_COMMAND} -E remove_directory includes
  DEPENDS remove
)

