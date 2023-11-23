// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'misc_example.freezed.dart';

Future<MyTreeNode> handleComplexStruct({required MyTreeNode s, dynamic hint}) =>
    RustLib.instance.api.handleComplexStruct(s: s, hint: hint);

Future<List<Weekdays>> listOfPrimitiveEnums(
        {required List<Weekdays> weekdays, dynamic hint}) =>
    RustLib.instance.api.listOfPrimitiveEnums(weekdays: weekdays, hint: hint);

Future<MyNestedStruct> handleNestedStruct(
        {required MyNestedStruct s, dynamic hint}) =>
    RustLib.instance.api.handleNestedStruct(s: s, hint: hint);

Future<BigBuffers> handleBigBuffers({dynamic hint}) =>
    RustLib.instance.api.handleBigBuffers(hint: hint);

Future<Abc> testAbcEnum({required Abc abc, dynamic hint}) =>
    RustLib.instance.api.testAbcEnum(abc: abc, hint: hint);

Future<ContainsMirroredSubStruct> testContainsMirroredSubStruct(
        {dynamic hint}) =>
    RustLib.instance.api.testContainsMirroredSubStruct(hint: hint);

Future<StructWithEnum> testStructWithEnum(
        {required StructWithEnum se, dynamic hint}) =>
    RustLib.instance.api.testStructWithEnum(se: se, hint: hint);

class A {
  final String a;

  const A({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is A && runtimeType == other.runtimeType && a == other.a;
}

@freezed
sealed class Abc with _$Abc {
  const factory Abc.a(
    A field0,
  ) = Abc_A;
  const factory Abc.b(
    B field0,
  ) = Abc_B;
  const factory Abc.c(
    C field0,
  ) = Abc_C;
  const factory Abc.justInt(
    int field0,
  ) = Abc_JustInt;
}

class B {
  final int b;

  const B({
    required this.b,
  });

  @override
  int get hashCode => b.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is B && runtimeType == other.runtimeType && b == other.b;
}

class BigBuffers {
  final Int64List int64;
  final Uint64List uint64;

  const BigBuffers({
    required this.int64,
    required this.uint64,
  });

  @override
  int get hashCode => int64.hashCode ^ uint64.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BigBuffers &&
          runtimeType == other.runtimeType &&
          int64 == other.int64 &&
          uint64 == other.uint64;
}

class C {
  final bool c;

  const C({
    required this.c,
  });

  @override
  int get hashCode => c.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is C && runtimeType == other.runtimeType && c == other.c;
}

class ContainsMirroredSubStruct {
  final RawStringMirrored test;
  final A test2;

  const ContainsMirroredSubStruct({
    required this.test,
    required this.test2,
  });

  @override
  int get hashCode => test.hashCode ^ test2.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ContainsMirroredSubStruct &&
          runtimeType == other.runtimeType &&
          test == other.test &&
          test2 == other.test2;
}

class MyNestedStruct {
  final MyTreeNode treeNode;
  final Weekdays weekday;

  const MyNestedStruct({
    required this.treeNode,
    required this.weekday,
  });

  @override
  int get hashCode => treeNode.hashCode ^ weekday.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MyNestedStruct &&
          runtimeType == other.runtimeType &&
          treeNode == other.treeNode &&
          weekday == other.weekday;
}

class MyTreeNode {
  final int valueI32;
  final Uint8List valueVecU8;
  final bool valueBoolean;
  final List<MyTreeNode> children;

  const MyTreeNode({
    required this.valueI32,
    required this.valueVecU8,
    required this.valueBoolean,
    required this.children,
  });

  @override
  int get hashCode =>
      valueI32.hashCode ^
      valueVecU8.hashCode ^
      valueBoolean.hashCode ^
      children.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MyTreeNode &&
          runtimeType == other.runtimeType &&
          valueI32 == other.valueI32 &&
          valueVecU8 == other.valueVecU8 &&
          valueBoolean == other.valueBoolean &&
          children == other.children;
}

class StructWithEnum {
  final Abc abc1;
  final Abc abc2;

  const StructWithEnum({
    required this.abc1,
    required this.abc2,
  });

  @override
  int get hashCode => abc1.hashCode ^ abc2.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithEnum &&
          runtimeType == other.runtimeType &&
          abc1 == other.abc1 &&
          abc2 == other.abc2;
}

enum Weekdays {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}
