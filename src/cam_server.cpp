#include <opencv2/imgproc/imgproc.hpp>
#include <opencv2/core/core.hpp>
#include <cameraserver/CameraServer.h>
#include <frc/TimedRobot.h>
#include <frc/smartdashboard/SmartDashboard.h>
#include <thread>

class Robot : public frc::TimedRobot {
 public:
  void RobotInit() override {
    // We need to run our vision program in a separate thread. If not, our robot
    // program will not run.
    std::thread visionThread([]() {
      // Get the UsbCamera from CameraServer
      cs::UsbCamera camera = frc::CameraServer::StartAutomaticCapture();
      // Set the resolution
      camera.SetResolution(640, 480);

      // Get a CvSink. This grabs images from the camera
      cs::CvSink cvSink = frc::CameraServer::GetVideo();
      // Setup a CvSource. This is where processed images will go
      cs::CvSource outputStream = frc::CameraServer::PutVideo("Processed", 640, 480);

      // Mats are very memory expensive. Reuse these Mats.
      cv::Mat mat;
      cv::Mat grayMat;

      while (true) {
        // Tell the CvSink to grab a frame from the camera and put it
        // in the source mat. If there is an error notify the output.
        if (cvSink.GrabFrame(mat) == 0) {
          // Send the output the error.
          outputStream.NotifyError(cvSink.GetError());
          // skip the rest of the current iteration
          continue;
        }

        // Put your image processing code here.
        // For example, convert to grayscale:
        cv::cvtColor(mat, grayMat, cv::COLOR_BGR2GRAY);

        // Give the output stream a new image to display
        outputStream.PutFrame(grayMat);
      }
    });
    // Make sure the vision thread doesn't get destroyed when the main thread terminates
    visionThread.detach();
  }
};


int main() {
  return frc::StartRobot<Robot>();
}

