// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'event_listener.freezed.dart';

Stream<Event> registerEventListener({dynamic hint}) =>
    RustLib.instance.api.registerEventListener(hint: hint);

Future<void> closeEventListener({dynamic hint}) =>
    RustLib.instance.api.closeEventListener(hint: hint);

Future<void> createEvent(
        {required String address, required String payload, dynamic hint}) =>
    RustLib.instance.api
        .createEvent(address: address, payload: payload, hint: hint);

@freezed
class Event with _$Event {
  const Event._();
  const factory Event({
    required String address,
    required String payload,
  }) = _Event;
  Future<String> asString({dynamic hint}) => RustLib.instance.api.eventAsString(
        that: this,
      );
}
