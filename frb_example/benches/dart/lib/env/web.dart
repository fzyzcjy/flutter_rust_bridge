// see https://github.com/flutter/flutter/issues/55870

import 'package:flutter_rust_bridge_benchmark/constants.dart';

final sampleCount = int.fromEnvironment('SAMPLE_COUNT', defaultValue: defaultSampleCount);
final warmUpTime = int.fromEnvironment('WARM_UP_TIME', defaultValue: defaultWarmUpTime);
final measurementTime = int.fromEnvironment('WARM_UP_TIME', defaultValue: defaultMeasurementTime);
final itemsCount = int.fromEnvironment('ITEMS_COUNT', defaultValue: defaultItemsCount);
const useJSON = bool.fromEnvironment('JSON');
