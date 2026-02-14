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
CLD2_INCLUDE_DIR = $(SRC_DIR)/cld2/lib/include

# Target files
TARGET = stemmer
TARGET_DIR = $(ROOT_DIR)
STEMMER_OBJ = $(SRC_DIR)/stemmer/stremmer.o

# Libraries
SNOWBALL_LIB = $(LIB_DIR)/libstemmer.a
CLD2_LIB = $(CLD2_LIB_DIR)/libcld2.so

# ============================================================================
# Main targets
# ============================================================================

all: dirs snowball cld2 headers stemmer
	@echo "✅ All built! Run: ./$(TARGET)"

# Create necessary directories
dirs:
	@mkdir -p $(LIB_DIR)/include
	@mkdir -p $(CLD2_LIB_DIR)
	@mkdir -p $(CLD2_INCLUDE_DIR)
	@mkdir -p $(CLD2_INCLUDE_DIR)/internal
	@echo "📁 Directories created"

# ============================================================================
# Build Snowball (stemmer)
# ============================================================================

snowball: 
	@echo "🔨 Building Snowball..."
	@cd $(SNOWBALL_SRC_DIR) && \
		$(MAKE) clean && \
		$(MAKE) libstemmer.a
	@cp $(SNOWBALL_SRC_DIR)/libstemmer.a $(LIB_DIR)/
	@cp $(SNOWBALL_SRC_DIR)/include/libstemmer.h $(INCLUDE_DIR)/
	@echo "✅ Snowball built"

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
	@echo "✅ CLD2 built: $(CLD2_LIB)"

# Copy CLD2 headers (including internal ones)
headers: cld2
	@echo "📋 Copying CLD2 headers..."
	@cp $(CLD2_SRC_DIR)/public/*.h $(CLD2_INCLUDE_DIR)/
	@cp $(CLD2_SRC_DIR)/internal/*.h $(CLD2_INCLUDE_DIR)/internal/
	@echo "✅ Headers copied"

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
		-I$(CLD2_INCLUDE_DIR) \
		-c $< -o $@

# ============================================================================
# Build CLD2 (your cld2_main.cpp)
# ============================================================================

CLD2_TARGET = cld2_main
CLD2_SRC = $(SRC_DIR)/cld2/cld2_main.cpp

cld2: $(CLD2_SRC)
	$(CXX) $(CXXFLAGS) \
		-I$(CLD2_INCLUDE_DIR) \
		-I$(CLD2_INCLUDE_DIR)/internal \
		$(CLD2_SRC) \
		-L$(CLD2_LIB_DIR) \
		-lcld2 \
		-Wl,-rpath,$(CLD2_LIB_DIR) \
		-o $(ROOT_DIR)/$(CLD2_TARGET)
	@echo "✅ CLD2 built: ./$(CLD2_TARGET)"
	@echo "▶️  Run with: ./$(CLD2_TARGET)"
	
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
	rm -rf $(LIB_DIR)/* $(CLD2_LIB_DIR)/* $(CLD2_INCLUDE_DIR)/*
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
	@echo "  headers     - copy CLD2 headers (including internal)"
	@echo "  stemmer     - build only the stemmer"
	@echo "  clean       - clean build files"
	@echo "  distclean   - full clean (including libraries)"
	@echo "  test        - run the test"
	@echo "  install-deps - install dependencies"
	@echo "  version     - show versions"
	@echo "  help        - this help"
	@echo "  cld2   - build CLD2 test program"

.PHONY: all dirs snowball cld2 headers stemmer clean distclean test install-deps version help