# C++ Development Environment Setup for macOS

Complete guide to setting up a modern C++ development environment on macOS.

## Prerequisites

- macOS 10.15 (Catalina) or later
- Admin access to install software
- Internet connection

## 1. Install Command Line Tools

First, install Xcode Command Line Tools which includes essential build tools:

```bash
# Install Xcode Command Line Tools
xcode-select --install
```

Verify installation:
```bash
# Check if tools are installed
xcode-select -p
# Should output: /Applications/Xcode.app/Contents/Developer or /Library/Developer/CommandLineTools

# Verify clang compiler
clang --version
```

## 2. Install Homebrew Package Manager

Homebrew makes installing development tools much easier:

```bash
# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Add Homebrew to PATH (follow the instructions shown after installation)
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
source ~/.zshrc

# Verify installation
brew --version
```

## 3. Install Modern C++ Compilers

### GCC (GNU Compiler Collection)
```bash
# Install latest GCC
brew install gcc

# Verify installation
gcc-13 --version  # Version number may vary
g++-13 --version
```

### Clang/LLVM (Alternative/Additional)
```bash
# Install latest LLVM/Clang
brew install llvm

# Add to PATH
echo 'export PATH="/opt/homebrew/opt/llvm/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Verify
clang++ --version
```

## 4. Install Build Systems

### CMake (Essential for modern C++)
```bash
# Install CMake
brew install cmake

# Verify
cmake --version
```

### Ninja (Fast build system)
```bash
# Install Ninja
brew install ninja

# Verify
ninja --version
```

### Make (Traditional build system)
```bash
# Usually comes with Command Line Tools, but verify:
make --version
```

## 5. Install Package Managers

### Conan (C++ Package Manager)
```bash
# Install Conan
pip3 install conan

# Verify
conan --version

# Create default profile
conan profile detect --force
```

### vcpkg (Microsoft's C++ Package Manager)
```bash
# Clone and install vcpkg
cd ~/
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
./bootstrap-vcpkg.sh

# Add to PATH
echo 'export PATH="$HOME/vcpkg:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Verify
vcpkg version
```

## 6. Install Development Tools

### Git (Version Control)
```bash
# Install Git
brew install git

# Configure Git
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# Verify
git --version
```

### Debugging Tools
```bash
# Install GDB debugger
brew install gdb

# Install Valgrind (memory debugging)
brew install valgrind

# Verify
gdb --version
valgrind --version
```

### Static Analysis Tools
```bash
# Install cppcheck
brew install cppcheck

# Install clang-tidy (comes with LLVM)
# Already installed if you installed llvm above

# Verify
cppcheck --version
clang-tidy --version
```

## 7. Choose and Install an IDE/Editor

### Option A: Visual Studio Code (Recommended for beginners)
```bash
# Install VS Code
brew install --cask visual-studio-code

# Essential C++ extensions (install after opening VS Code):
# - C/C++ Extension Pack (ms-vscode.cpptools-extension-pack)
# - CMake Tools (ms-vscode.cmake-tools)
# - Code Runner (formulahendry.code-runner)
```

### Option B: CLion (JetBrains - Professional)
```bash
# Install CLion
brew install --cask clion
```

### Option C: Xcode (Apple's IDE)
```bash
# Install full Xcode from App Store
# or via command line:
xcode-select --install
```

### Option D: Vim/Neovim (Advanced users)
```bash
# Install Neovim
brew install neovim

# Popular C++ plugins:
# - coc.nvim or nvim-lsp for language server
# - vim-cpp-modern for syntax highlighting
```

## 8. Set Up a Test Project

Create a simple C++ project to verify your setup:

```bash
# Create project directory
mkdir ~/cpp-projects
cd ~/cpp-projects
mkdir hello-world
cd hello-world

# Create main.cpp
cat > main.cpp << 'EOF'
#include <iostream>
#include <vector>
#include <string>

int main() {
    std::vector<std::string> greetings = {
        "Hello, World!",
        "Welcome to C++!",
        "Your dev environment is ready!"
    };
    
    for (const auto& greeting : greetings) {
        std::cout << greeting << std::endl;
    }
    
    return 0;
}
EOF

# Create CMakeLists.txt
cat > CMakeLists.txt << 'EOF'
cmake_minimum_required(VERSION 3.15)
project(HelloWorld)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(hello_world main.cpp)
EOF
```

## 9. Build and Run Test Project

### Method 1: Direct compilation
```bash
# Compile with g++
g++-13 -std=c++20 -Wall -Wextra -O2 main.cpp -o hello_world

# Run
./hello_world
```

### Method 2: Using CMake
```bash
# Create build directory
mkdir build
cd build

# Configure with CMake
cmake ..

# Build
make

# Run
./hello_world
```

### Method 3: Using Ninja
```bash
# Configure with CMake and Ninja
cmake -G Ninja ..

# Build
ninja

# Run
./hello_world
```

## 10. Install Additional Libraries (Optional)

### Boost Libraries
```bash
brew install boost
```

### Google Test (Unit Testing)
```bash
brew install googletest
```

### SFML (Graphics/Game Development)
```bash
brew install sfml
```

### OpenCV (Computer Vision)
```bash
brew install opencv
```

## 11. Configuration Files

### Create .clang-format for code formatting
```bash
cat > ~/.clang-format << 'EOF'
BasedOnStyle: Google
IndentWidth: 4
ColumnLimit: 100
EOF
```

### Create .gitignore for C++ projects
```bash
cat > ~/cpp-projects/.gitignore << 'EOF'
# Build directories
build/
cmake-build-*/

# Compiled Object files
*.o
*.obj

# Executables
*.exe
*.out
*.app

# Libraries
*.lib
*.a
*.so
*.dylib

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS generated files
.DS_Store
Thumbs.db
EOF
```

## 12. Verification Checklist

Run these commands to verify your setup:

```bash
# Compilers
g++-13 --version
clang++ --version

# Build systems
cmake --version
ninja --version
make --version

# Package managers
conan --version
vcpkg version

# Tools
git --version
gdb --version
cppcheck --version

# Test compilation
cd ~/cpp-projects/hello-world
g++-13 -std=c++20 main.cpp -o test && ./test
```

## 13. Next Steps for Learning C++

### Recommended Learning Resources
1. **Books**:
   - "A Tour of C++" by Bjarne Stroustrup
   - "Effective Modern C++" by Scott Meyers
   - "C++ Primer" by Stanley Lippman

2. **Online Courses**:
   - learncpp.com
   - cppreference.com
   - Coursera C++ courses

3. **Practice Platforms**:
   - LeetCode
   - HackerRank
   - Codeforces

### Project Ideas for Practice
1. Command-line calculator
2. File organizer utility
3. Simple game (Tic-tac-toe, Snake)
4. Data structure implementations
5. Web scraper using libcurl

## Troubleshooting

### Common Issues

**Issue**: `command not found` errors
**Solution**: Make sure Homebrew is in your PATH:
```bash
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
source ~/.zshrc
```

**Issue**: CMake can't find compiler
**Solution**: Set compiler explicitly:
```bash
export CC=gcc-13
export CXX=g++-13
```

**Issue**: Permission denied when installing
**Solution**: Don't use sudo with Homebrew. Reinstall Homebrew if needed.

---

**Your C++ development environment is now ready!** 🎉

Start with simple programs and gradually work your way up to more complex projects. Happy coding!