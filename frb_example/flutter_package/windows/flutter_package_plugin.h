#ifndef FLUTTER_PLUGIN_FLUTTER_PACKAGE_PLUGIN_H_
#define FLUTTER_PLUGIN_FLUTTER_PACKAGE_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace flutter_package {

class FlutterPackagePlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  FlutterPackagePlugin();

  virtual ~FlutterPackagePlugin();

  // Disallow copy and assign.
  FlutterPackagePlugin(const FlutterPackagePlugin&) = delete;
  FlutterPackagePlugin& operator=(const FlutterPackagePlugin&) = delete;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace flutter_package

#endif  // FLUTTER_PLUGIN_FLUTTER_PACKAGE_PLUGIN_H_
