// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'ci_plan.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Map<String, dynamic> _$CiPlanOutputToJson(CiPlanOutput instance) =>
    <String, dynamic>{
      'deploy_website': instance.deployWebsite.toJson(),
      'lint_rust_primary': instance.lintRustPrimary.toJson(),
      'lint_dart_primary': instance.lintDartPrimary.toJson(),
      'lint_rust_feature_flag': instance.lintRustFeatureFlag.toJson(),
      'generate_run_frb_codegen_command_generate': instance
          .generateRunFrbCodegenCommandGenerate
          .toJson(),
      'generate_run_frb_codegen_command_integrate': instance
          .generateRunFrbCodegenCommandIntegrate
          .toJson(),
      'generate_internal': instance.generateInternal.toJson(),
      'bench_dart_native': instance.benchDartNative.toJson(),
      'bench_upload': instance.benchUpload.toJson(),
      'build_flutter': instance.buildFlutter.toJson(),
      'test_mimic_quickstart': instance.testMimicQuickstart.toJson(),
      'test_rust': instance.testRust.toJson(),
      'test_dart_native': instance.testDartNative.toJson(),
      'test_dart_web': instance.testDartWeb.toJson(),
      'test_dart_valgrind': instance.testDartValgrind.toJson(),
      'test_dart_sanitizer': instance.testDartSanitizer.toJson(),
      'test_flutter_native_android': instance.testFlutterNativeAndroid.toJson(),
      'test_flutter_native_ios': instance.testFlutterNativeIos.toJson(),
      'test_flutter_native_desktop': instance.testFlutterNativeDesktop.toJson(),
      'test_flutter_web': instance.testFlutterWeb.toJson(),
      'misc_codecov': instance.miscCodecov.toJson(),
    };

Map<String, dynamic> _$CiPlanJobOutputToJson(CiPlanJobOutput instance) =>
    <String, dynamic>{
      'enable': instance.enable,
      if (instance.matrix case final value?) 'matrix': value,
    };
