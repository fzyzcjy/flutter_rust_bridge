import UIKit
import Flutter

@UIApplicationMain
@objc class AppDelegate: FlutterAppDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    GeneratedPluginRegistrant.register(with: self)
    print("dummy_value=\(dummy_method_to_enforce_bundling())");
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
}
