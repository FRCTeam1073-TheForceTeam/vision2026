# Define variables for the compiler and flags
CXX := g++
CXXFLAGS := -Wall -Wextra -std=c++20 -Iinclude -L/usr/local/lib
LIBS := -lntcore -lwpiutil

NT_INCLUDES := -I/usr/local/include/ntcore -I/usr/local/include/wpiutil
NT_LIBS := -lntcore -lwpiutil

CS_INCLUDES := -I/usr/local/include/wpilibc -I/usr/local/include/hal -I/usr/local/include/wpimath -I/usr/include/opencv4 -I/usr/local/include/cscore -I/usr/local/include/cameraserver -I/usr/local/include/wpiutil
CS_LIBS := -lcameraserver -lcscore  -lopencv_imgproc -lopencv_core -lwpilibc -lwpiHal


skeleton: src/skeleton.cpp
	$(CXX) $(CXXFLAGS) -o skeleton src/skeleton.cpp $(NT_INCLUDES) $(NT_LIBS)

cam_server: src/cam_server.cpp
	$(CXX) $(CXXFLAGS) -o cam_server src/cam_server.cpp $(NT_INCLUDES) $(CS_INCLUDES) $(CS_LIBS) $(NT_LIBS)


# Target to clean up generated files
clean:
	rm -f $(TARGET) $(OBJS)
