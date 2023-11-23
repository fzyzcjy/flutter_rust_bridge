// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../auxiliary/new_module_system/sub_module.dart';
import '../auxiliary/old_module_system/sub_module.dart';
import '../auxiliary/sample_types.dart';
import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<bool> useImportedStruct({required MyStruct myStruct, dynamic hint}) =>
    RustLib.instance.api.useImportedStruct(myStruct: myStruct, hint: hint);

Future<bool> useImportedEnum({required MyEnum myEnum, dynamic hint}) =>
    RustLib.instance.api.useImportedEnum(myEnum: myEnum, hint: hint);

Future<OldSimpleStruct> callOldModuleSystem({dynamic hint}) =>
    RustLib.instance.api.callOldModuleSystem(hint: hint);

Future<NewSimpleStruct> callNewModuleSystem({dynamic hint}) =>
    RustLib.instance.api.callNewModuleSystem(hint: hint);
