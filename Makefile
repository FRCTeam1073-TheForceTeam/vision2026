# Define variables for the compiler and flags
CXX := g++
CXXFLAGS := -Wall -Wextra -std=c++17 -Iinclude
TARGET := main
SRCS := src/$(wildcard *.cpp)
OBJS := $(SRCS:.cpp=.o)

# Default target (builds the executable)
all: $(TARGET)

# Rule to link object files into the final executable
$(TARGET): $(OBJS)
	$(CXX) $(CXXFLAGS) $(OBJS) -o $(TARGET)

# Pattern rule to compile .cpp files into .o files
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Target to run the program
run: all
	./$(TARGET)

# Target to clean up generated files
clean:
	rm -f $(TARGET) $(OBJS)
