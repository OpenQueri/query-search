# ============================================================================
# Query Search Engine Makefile
# ============================================================================

# Compiler and flags
CXX = g++
CXXFLAGS = -std=c++20 -O2 -Wall -pthread
AR = ar
ARFLAGS = rcs

# Directories
ROOT_DIR = $(shell pwd)
SRC_DIR = $(ROOT_DIR)/src/cpp
STEMMER_SRC_DIR = $(SRC_DIR)/stemmer
CLD2_SRC_DIR = $(ROOT_DIR)/cld2
SNOWBALL_SRC_DIR = $(ROOT_DIR)/snowball

# Library directories
LIB_DIR = $(SRC_DIR)/stemmer/lib
CLD2_LIB_DIR = $(SRC_DIR)/cld2/lib
INCLUDE_DIR = $(LIB_DIR)/include

# Target files
TARGET = stemmer
STEMMER_OBJ = $(SRC_DIR)/stemmer/stremmer.o

# Libraries
SNOWBALL_LIB = $(LIB_DIR)/libstemmer.a
CLD2_LIB = $(CLD2_LIB_DIR)/libcld2.so

# ============================================================================
# Main targets
# ============================================================================

all: dirs snowball cld2 stemmer
	@echo "✅ All built! Run: ./$(TARGET)"

# Create necessary directories
dirs:
	@mkdir -p $(LIB_DIR)/include
	@mkdir -p $(CLD2_LIB_DIR)
	@mkdir -p $(SRC_DIR)/cld2/lib/include
	@echo "📁 Directories created"

# ============================================================================
# Build Snowball (stemmer)
# ============================================================================

snowball: $(SNOWBALL_LIB)

$(SNOWBALL_LIB):
	@echo "🔨 Building Snowball..."
	@cd $(SNOWBALL_SRC_DIR) && \
		$(MAKE) clean && \
		$(MAKE) libstemmer.a
	@cp $(SNOWBALL_SRC_DIR)/libstemmer.a $(LIB_DIR)/
	@cp $(SNOWBALL_SRC_DIR)/include/libstemmer.h $(INCLUDE_DIR)/
	@echo "✅ Snowball built: $(SNOWBALL_LIB)"

# ============================================================================
# Build CLD2 (language detector)
# ============================================================================

cld2: $(CLD2_LIB)

$(CLD2_LIB):
	@echo "🔨 Building CLD2..."
	@cd $(CLD2_SRC_DIR)/internal && \
		CXXFLAGS="-Wno-narrowing" ./compile_libs.sh
	@cp $(CLD2_SRC_DIR)/internal/libcld2*.so $(CLD2_LIB_DIR)/
	@cp $(CLD2_SRC_DIR)/public/*.h $(SRC_DIR)/cld2/lib/include/
	@echo "✅ CLD2 built: $(CLD2_LIB)"

# ============================================================================
# Build stemmer (your C++ code)
# ============================================================================

stemmer: $(STEMMER_OBJ)
	@echo "🔨 Linking stemmer..."
	$(CXX) $(STEMMER_OBJ) \
		-L$(LIB_DIR) -L$(CLD2_LIB_DIR) \
		-lstemmer -lcld2 \
		-Wl,-rpath,$(LIB_DIR) -Wl,-rpath,$(CLD2_LIB_DIR) \
		$(CXXFLAGS) -o $(TARGET)
	@echo "✅ Stemmer ready: ./$(TARGET)"

$(STEMMER_OBJ): $(STEMMER_SRC_DIR)/stremmer.cpp
	@echo "🔨 Compiling stemmer..."
	$(CXX) $(CXXFLAGS) \
		-I$(INCLUDE_DIR) \
		-I$(SRC_DIR)/cld2/lib/include \
		-c $< -o $@

# ============================================================================
# Helper targets
# ============================================================================

# Clean everything
clean:
	@echo "🧹 Cleaning..."
	rm -rf $(STEMMER_OBJ) $(TARGET)
	@cd $(SNOWBALL_SRC_DIR) && $(MAKE) clean
	@cd $(CLD2_SRC_DIR)/internal && rm -f *.o *.so
	@echo "✅ Cleaned"

# Clean including libraries
distclean: clean
	@echo "🧹 Full clean..."
	rm -rf $(LIB_DIR)/* $(CLD2_LIB_DIR)/*
	@echo "✅ Fully cleaned"

# Testing
test: $(TARGET)
	@echo "🧪 Testing..."
	./$(TARGET)

# Install dependencies (for Fedora)
install-deps:
	@echo "📦 Installing dependencies..."
	sudo dnf install -y gcc-c++ make
	@echo "✅ Dependencies installed"

# Show versions
version:
	@echo "🔍 Versions:"
	@$(CXX) --version | head -n1
	@echo "Make: $(MAKE_VERSION)"

# Help
help:
	@echo "Available targets:"
	@echo "  all         - build everything (default)"
	@echo "  snowball    - build only Snowball"
	@echo "  cld2        - build only CLD2"
	@echo "  stemmer     - build only the stemmer"
	@echo "  clean       - clean build files"
	@echo "  distclean   - full clean (including libraries)"
	@echo "  test        - run the test"
	@echo "  install-deps - install dependencies"
	@echo "  version     - show versions"
	@echo "  help        - this help"

.PHONY: all dirs snowball cld2 stemmer clean distclean test install-deps version help