# Define variables for the compiler and flags
CXX := g++
CXXFLAGS := -Wall -Wextra -std=c++20 -Iinclude -I/usr/local/include/ntcore -I/usr/local/include/wpiutil -L/usr/local/lib
LIBS := -lntcore -lwpiutil

skeleton: src/skeleton.cpp
	$(CXX) $(CXXFLAGS) -o skeleton src/skeleton.cpp $(LIBS)

# Target to clean up generated files
clean:
	rm -f $(TARGET) $(OBJS)
