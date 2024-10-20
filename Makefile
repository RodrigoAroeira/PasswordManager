# Directories
SRC_DIR = src
HEADER_DIR = headers
INCLUDE_DIR = includes/usr/include
PKG_CONFIG_DIR = includes/usr/lib/x86_64-linux-gnu

# Source and Output Files
SRC_FILES = $(wildcard $(SRC_DIR)/*.cpp)
OUT = PasswordManager.exe

# PKG Config
PKG_CONFIG_PATH := $(PKG_CONFIG_DIR)/pkgconfig
PKG_CFLAGS = $(shell PKG_CONFIG_PATH=$(PKG_CONFIG_PATH) pkg-config --cflags sqlite3 openssl)
PKG_LIBS = $(shell PKG_CONFIG_PATH=$(PKG_CONFIG_PATH) pkg-config --libs sqlite3 openssl)

# Compilation Flags
CPPFLAGS = -I$(HEADER_DIR) -I$(INCLUDE_DIR) $(PKG_CFLAGS)
LDFLAGS = -L$(PKG_CONFIG_DIR) $(PKG_LIBS)

# Targets
.PHONY: build run all clean veryclean deps

build:
	g++ $(SRC_FILES) -o $(OUT) $(CPPFLAGS) $(LDFLAGS)

run:
	./$(OUT)

all: deps build

clean:
	rm -rf $(OUT)

veryclean: clean
	rm -rf ./includes/

# New target to install dependencies locally
deps: includes/sqlite3.h includes/openssl/ssl.h

includes/sqlite3.h:
	mkdir -p includes
	@if [ ! -f includes/usr/include/sqlite3.h ]; then \
		apt-get download libsqlite3-dev; \
		dpkg-deb -xv libsqlite3-dev*.deb includes; \
		rm libsqlite3-dev*.deb; \
	fi

includes/openssl/ssl.h:
	mkdir -p includes
	@if [ ! -f includes/usr/include/openssl/ssl.h ]; then \
		apt-get download libssl-dev; \
		dpkg-deb -xv libssl-dev*.deb includes; \
		rm libssl-dev*.deb; \
	fi
