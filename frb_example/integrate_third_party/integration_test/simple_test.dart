import 'package:flutter_test/flutter_test.dart';
import 'package:frb_example_integrate_third_party/src/rust/frb_generated.dart';
import 'package:frb_example_integrate_third_party/src/rust/third_party/web_audio_api/context.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());

  test('empty test', () {});
}

// Only write this down but do not call it, since the Rust side deliberately
// compiles without background library to speed up, and this flutter_rust_bridge
// test package does not aim to provide a ready-to-use-in-production binding,
// but only to test code generator functionality.
//
// The following is adapted from `web-audio-api`'s readme demo.
//
// ignore: unused_element
Future<void> _demoUsage() async {
  const options = AudioContextOptions(
    latencyHint: AudioContextLatencyCategory.balanced(),
    sinkId: '',
    renderSizeHint: AudioContextRenderSizeCategory.default_,
  );
  final context = AudioContext(options: options);

  final buffer =
      await context.decodeAudioDataSync(inputPath: 'samples/major-scale.ogg');

  final src = await context.createBufferSource();
  await src.setBuffer(audioBuffer: buffer);
  await src.setLoop(value: true);

  final biquad = await context.createBiquadFilter();
  biquad.frequency.setValue(value: 125);

  await src.connect(dest: biquad);
  await biquad.connect(dest: await context.destination());

  await src.start();

  await Future.delayed(const Duration(seconds: 4));
}
