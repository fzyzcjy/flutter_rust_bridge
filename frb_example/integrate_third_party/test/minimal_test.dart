import 'package:frb_example_integrate_third_party/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('empty test', () {});

  // TODO
  // test('demo usage', () async {
  //   final context = AudioContext(
  //     options: const AudioContextOptions(
  //       latencyHint: AudioContextLatencyCategory.balanced(),
  //       sinkId: '',
  //       renderSizeHint: AudioContextRenderSizeCategory.Default,
  //     ),
  //   );
  //
  //   final buffer =
  //       await context.decodeAudioDataSync(inputPath: 'samples/major-scale.ogg');
  //
  //   final src = await context.createBufferSource();
  //   src.setBuffer(buffer);
  //   src.setLoop(true);
  //
  //   final biquad = context.createBiquadFilter();
  //   biquad.frequencyValue = 125;
  //
  //   await src.conect(biquad);
  //   await biquad.conect(await context.destination());
  //
  //   await src.start();
  //
  //   await Future.delayed(const Duration(seconds: 4));
  // });
}
