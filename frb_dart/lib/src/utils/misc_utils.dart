/// {@macro flutter_rust_bridge.only_for_generated_code}
List<T?> mapNonNull<T, I>(List<I?> items, T Function(I) mapper) {
  final out = List<T?>.filled(items.length, null);
  for (var i = 0; i < items.length; ++i) {
    final item = items[i];
    if (item != null) out[i] = mapper(item);
  }
  return out;
}
