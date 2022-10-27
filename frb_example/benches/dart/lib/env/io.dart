// see https://github.com/flutter/flutter/issues/55870

import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_benchmark/constants.dart';

final sampleCount = int.parse(Platform.environment['SAMPLE_COUNT'] ?? defaultSampleCount.toString());
final warmUpTime = int.parse(Platform.environment['WARM_UP_TIME'] ?? defaultWarmUpTime.toString());
final measurementTime = int.parse(Platform.environment['MEASUREMENT_TIME'] ?? defaultMeasurementTime.toString());
final itemsCount = int.parse(Platform.environment['ITEMS_COUNT'] ?? defaultItemsCount.toString());
final outputFilename = Platform.environment['OUTPUT_FILENAME'] ?? defaultOutputFilename;
