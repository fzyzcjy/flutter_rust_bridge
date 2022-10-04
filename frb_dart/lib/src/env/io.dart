import 'dart:io';

bool useJSON() =>
    (Platform.environment["JSON"] ?? 'false') ==
    'true'; // see https://github.com/flutter/flutter/issues/55870
