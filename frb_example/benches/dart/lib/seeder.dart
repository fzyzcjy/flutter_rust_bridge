import 'dart:math' as math;

import './utils.dart';
import 'package:uuid/uuid.dart';

const int just_1000 = 1000;
const int just_100_000 = 100000;

final math.Random random = math.Random(1000);
final Uuid uuid = Uuid();

abstract class Seeder<T> {
  T get seed;
}

class UuidSeeder extends Seeder<UuidValue> {
  @override
  UuidValue get seed => uuid.v4obj();
}

class StringSeeder extends Seeder<String> {
  @override
  String get seed => getRandomString(1000);
}

class IntSeeder extends Seeder<int> {
  @override
  int get seed => random.nextInt(1000);
}

class DoubleSeeder extends Seeder<double> {
  @override
  double get seed => random.nextDouble();
}

List<T> seed<T>(Seeder seeder, int howMany, {bool? growable}) {
  final seeds = List<T>.generate(howMany, (_) => seeder.seed, growable: growable ?? false);
  return seeds;
}
