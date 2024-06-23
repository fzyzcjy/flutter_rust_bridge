#include "include/flutter_package/flutter_package_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "flutter_package_plugin.h"

void FlutterPackagePluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  flutter_package::FlutterPackagePlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
