// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'misc_no_twin_example_a.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

MyEnumWithJsonSerializableTwinNormal_Apple
    _$MyEnumWithJsonSerializableTwinNormal_AppleFromJson(
            Map<String, dynamic> json) =>
        MyEnumWithJsonSerializableTwinNormal_Apple(
          json['field0'] as String,
          $type: json['runtimeType'] as String?,
        );

Map<String, dynamic> _$MyEnumWithJsonSerializableTwinNormal_AppleToJson(
        MyEnumWithJsonSerializableTwinNormal_Apple instance) =>
    <String, dynamic>{
      'field0': instance.field0,
      'runtimeType': instance.$type,
    };

MyEnumWithJsonSerializableTwinNormal_Orange
    _$MyEnumWithJsonSerializableTwinNormal_OrangeFromJson(
            Map<String, dynamic> json) =>
        MyEnumWithJsonSerializableTwinNormal_Orange(
          a: (json['a'] as num).toInt(),
          $type: json['runtimeType'] as String?,
        );

Map<String, dynamic> _$MyEnumWithJsonSerializableTwinNormal_OrangeToJson(
        MyEnumWithJsonSerializableTwinNormal_Orange instance) =>
    <String, dynamic>{
      'a': instance.a,
      'runtimeType': instance.$type,
    };

_MyStructWithJsonSerializableTwinNormal
    _$MyStructWithJsonSerializableTwinNormalFromJson(
            Map<String, dynamic> json) =>
        _MyStructWithJsonSerializableTwinNormal(
          fieldOne: json['fieldOne'] as String,
        );

Map<String, dynamic> _$MyStructWithJsonSerializableTwinNormalToJson(
        _MyStructWithJsonSerializableTwinNormal instance) =>
    <String, dynamic>{
      'fieldOne': instance.fieldOne,
    };

_MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal
    _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalFromJson(
            Map<String, dynamic> json) =>
        _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal(
          a: json['a'] as String,
        );

Map<String, dynamic>
    _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalToJson(
            _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal
                instance) =>
        <String, dynamic>{
          'a': instance.a,
        };
