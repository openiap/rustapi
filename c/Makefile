TARGET          := client_cli
SRCS            := main.c
OPENIAP_VERSION := 0.0.38
INCLUDE_DIR     := include
OBJS            := $(SRCS:.c=.o)

# Detect OS & architecture
UNAME_S         := $(shell uname -s)
ARCH            := $(shell uname -m)

ifeq ($(UNAME_S),Darwin)
    OS_SUFFIX   := macos
else ifeq ($(UNAME_S),Linux)
    OS_SUFFIX   := linux
else ifeq ($(UNAME_S),Windows_NT)
    OS_SUFFIX   := windows
else
    $(error Unsupported OS: $(UNAME_S))
endif

ifeq ($(ARCH),x86_64)
    ARCH_SUFFIX := x64
else ifeq ($(ARCH),amd64)
    ARCH_SUFFIX := x64
else ifeq ($(ARCH),arm64)
    ARCH_SUFFIX := arm64
else ifeq ($(ARCH),aarch64)
    ARCH_SUFFIX := arm64
else ifeq ($(ARCH),i686)
    ARCH_SUFFIX := i686
else ifeq ($(ARCH),i386)
    ARCH_SUFFIX := i686
else
    $(error Unsupported architecture: $(ARCH))
endif

# Compiler settings
CC              := gcc
CFLAGS          := -I. -Wall -Wextra -O2

# OpenIAP version and header URL
HEADER_URL      := https://raw.githubusercontent.com/openiap/rustapi/refs/tags/$(OPENIAP_VERSION)/crates/clib/clib_openiap.h

# Library output directory
LIB_DIR         := lib

# Dynamic linking flags
LDFLAGS         := -L$(LIB_DIR) -Wl,-rpath,'$$ORIGIN/$(LIB_DIR)'

# Library names
ifeq ($(OS_SUFFIX),macos)
    LIB_EXT := dylib
else ifeq ($(OS_SUFFIX),windows)
    LIB_EXT := dll
else
    LIB_EXT := so
endif

LIB_BASE        := openiap-$(OS_SUFFIX)-$(ARCH_SUFFIX)
LIB_SO          := $(LIB_DIR)/lib$(LIB_BASE).$(LIB_EXT)
LIB_GENERIC_SO  := $(LIB_DIR)/libopeniap_clib.$(LIB_EXT)

# Phony targets
.PHONY: all clean download_deps prepare_lib dockerbuild run

# Default build: dynamic
all: download_deps prepare_lib $(TARGET)
	@echo "Built $(TARGET) (dynamic)"

# Ensure the include directory exists
$(INCLUDE_DIR):
	@mkdir -p $(INCLUDE_DIR)

download_deps: $(INCLUDE_DIR)/clib_openiap.h $(LIB_SO)

$(INCLUDE_DIR)/clib_openiap.h: | $(INCLUDE_DIR)
	@echo "Downloading C header..."
	@curl -sSL -o $@ $(HEADER_URL)

$(LIB_DIR):
	@mkdir -p $(LIB_DIR)

$(LIB_SO): | $(LIB_DIR)
	@echo "Downloading OpenIAP shared library..."
	@curl -sSL -o $@ \
	  https://github.com/openiap/rustapi/releases/download/$(OPENIAP_VERSION)/lib$(LIB_BASE).$(LIB_EXT)
	@chmod +x $@ || true

# Copy to generic name so linker and loader see libopeniap_clib.dylib
prepare_lib: $(LIB_SO)
	@echo "Copying to generic name for loader..."
	@cp $(LIB_SO) $(LIB_GENERIC_SO)
ifeq ($(OS_SUFFIX),macos)
	@install_name_tool -id @rpath/libopeniap_clib.dylib $(LIB_GENERIC_SO)
endif

$(TARGET): $(OBJS) prepare_lib
	$(CC) $(OBJS) -o $(TARGET) $(LDFLAGS) -lopeniap_clib
ifeq ($(OS_SUFFIX),macos)
	@if [ -f $(TARGET) ]; then \
		echo "Patching $(TARGET) for macOS..."; \
		install_name_tool -add_rpath @executable_path/lib $(TARGET); \
	fi
else ifeq ($(OS_SUFFIX),linux)
	@if [ -f $(TARGET) ]; then \
		echo "Patching $(TARGET) for Linux..."; \
		patchelf --set-rpath '$$ORIGIN/lib' $(TARGET); \
	fi
endif

%.o: %.c $(INCLUDE_DIR)/clib_openiap.h
	$(CC) $(CFLAGS) -c $< -o $@

# Patch the interpreter on a NixOS-built binary
dockerbuild: all
	@command -v patchelf >/dev/null 2>&1 || { \
	  echo "Error: patchelf is required for dockerbuild. Install it locally."; \
	  exit 1; }
	@patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 $(TARGET)
	@echo "✓ $(TARGET) patched for native loader"

clean:
	@rm -rf $(TARGET) $(OBJS) $(INCLUDE_DIR) $(LIB_DIR)

run: $(TARGET)
	@./$(TARGET)

# Debug build
debug: clean $(OBJS) $(INCLUDE_DIR) $(LIB_DIR)
	@echo "Copying debug library to lib directory..."
	@mkdir -p $(LIB_DIR) ${INCLUDE_DIR}
	@cp ../target/debug/libopeniap_clib.so $(LIB_DIR)/libopeniap-linux-$(ARCH_SUFFIX).so
	@cp ../target/debug/libopeniap_clib.so $(LIB_DIR)/libopeniap_clib.so
	@cp ../crates/clib/clib_openiap.h $(INCLUDE_DIR)/clib_openiap.h
	$(CC) $(OBJS) -o $(TARGET) $(LDFLAGS) -l:libopeniap-linux-$(ARCH_SUFFIX).so
	@echo "Built $(TARGET) (debug)"
