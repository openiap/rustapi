# Detect the operating system.
UNAME_S := $(shell uname -s)

# Set the C compiler.
CC := gcc

# Determine build type.
ifdef DEBUG
    BUILD_TYPE := debug
    LIB_DIR := ../target/debug
    CFLAGS := -I. -Wall -Wextra -g -O0
else
    BUILD_TYPE := release
    LIB_DIR := lib
    CFLAGS := -I. -Wall -Wextra -O2
endif

# Set linker flags to use the appropriate lib directory with rpath.
LDFLAGS := -L$(LIB_DIR) -Wl,-rpath,$(LIB_DIR)

# Choose the library name based on OS and build type.
ifeq ($(BUILD_TYPE),debug)
    # Debug build uses the common library name across platforms.
    LIB := -lopeniap_clib
else
    ifeq ($(UNAME_S),Linux)
        LIB := -lopeniap-linux-x64
    else ifeq ($(UNAME_S),Darwin)
        LIB := -lopeniap-macos-x64
    else
        $(error Unsupported OS: $(UNAME_S))
    endif
endif

# Binary name and source files.
TARGET  := client_cli
SRCS    := main.c
OBJS    := $(SRCS:.c=.o)

.PHONY: all clean run debug

all: $(TARGET)
	@echo "Built $(TARGET) in $(BUILD_TYPE) mode."

$(TARGET): $(OBJS)
	$(CC) $(OBJS) $(LDFLAGS) $(LIB) -o $(TARGET)

%.o: %.c clib_openiap.h
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(TARGET) $(OBJS)

run: $(TARGET)
	./$(TARGET)

debug:
	$(MAKE) DEBUG=1 all
