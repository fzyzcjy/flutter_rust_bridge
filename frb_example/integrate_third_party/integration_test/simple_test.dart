import 'package:flutter_test/flutter_test.dart';
import 'package:frb_example_integrate_third_party/src/rust/api/media_element.dart';
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
// ignore: unused_element
Future<void> _rustWebAudioApiReadmeDemoUsage() async {
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
  biquad.frequency.value = 125;

  await src.connect(dest: biquad);
  await biquad.connect(dest: await context.destination());

  await src.start();

  await Future.delayed(const Duration(seconds: 4));
}

// The following is adapted from this demo:
// https://developer.mozilla.org/zh-CN/docs/Web/API/Web_Audio_API/Using_Web_Audio_API
// https://github.com/mdn/webaudio-examples/blob/main/audio-basics/index.html
// ignore: unused_element
Future<void> _mdnUsingWebAudioApiDemoUsage() async {
  // TODO default options
  const options = AudioContextOptions(
    latencyHint: AudioContextLatencyCategory.balanced(),
    sinkId: '',
    renderSizeHint: AudioContextRenderSizeCategory.default_,
  );
  final audioContext = AudioContext(options: options);

  final audioElement = MediaElement(file: 'some_file_here');
  final track =
      await audioContext.createMediaElementSource(mediaElement: audioElement);

  final gainNode = await audioContext.createGain();
  gainNode.gain.value = 1.2345;

  final panner = await audioContext.createStereoPanner();
  panner.pan.value = 1.2345;

  await track.connect(dest: gainNode);
  await gainNode.connect(dest: panner);
  await panner.connect(dest: await audioContext.destination());

  await audioContext.resumeSync();

  await audioElement.play();
  await audioElement.pause();
}
