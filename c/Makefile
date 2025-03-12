# Detect the operating system.
UNAME_S := $(shell uname -s)

# Set the C compiler.
CC := gcc

# Determine build type.
ifdef DEBUG
    BUILD_TYPE := debug
    LIB_DIR := ../target/debug
    CFLAGS := -Iinclude -Wall -Wextra -g -O0
else
    BUILD_TYPE := release
    LIB_DIR := lib
    CFLAGS := -Iinclude -Wall -Wextra -O2
endif

# Set linker flags to use the appropriate lib directory with rpath.
LDFLAGS := -L$(LIB_DIR) -Wl,-rpath,$(LIB_DIR)

# Choose the correct library name.
ifeq ($(BUILD_TYPE),debug)
    LIB := -lopeniap_clib
else
    ifeq ($(UNAME_S),Linux)
        LIB := -l:libopeniap-linux-x64.so
    else ifeq ($(UNAME_S),Darwin)
        LIB := -l:libopeniap-macos-x64.dylib
    else
        $(error Unsupported OS: $(UNAME_S))
    endif
endif

# Binary name and source files.
TARGET  := client_cli
SRCS    := test_package/main.c
OBJS    := $(SRCS:.c=.o)

.PHONY: all clean run debug package-c

all: $(TARGET)
	@echo "Built $(TARGET) in $(BUILD_TYPE) mode."

$(TARGET): $(OBJS)
	$(CC) $(OBJS) $(LDFLAGS) $(LIB) -o $(TARGET)

test_package/%.o: test_package/%.c include/clib_openiap.h
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(TARGET) $(OBJS)

run: $(TARGET)
	./$(TARGET)

debug:
	$(MAKE) DEBUG=1 all

package-c:
	conan create . -s os=Linux -s arch=x86_64
	conan create . -s os=Linux -s arch=armv8
	conan create . -s os=Macos -s arch=x86_64
	conan create . -s os=Macos -s arch=armv8
	conan create . -s os=Windows -s arch=x86
	conan create . -s os=Windows -s arch=x86_64
