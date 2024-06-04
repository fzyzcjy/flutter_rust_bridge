import 'consts.dart';

Future<void> installSystemDependencies({required String? package}) async {
  if (TODO) {
    await exec('sudo apt-get update');
    // https://github.com/RustAudio/cpal/blob/0246442da2f401895d2c82bbf941e3ebf6e93a04/.github/workflows/cpal.yml#L48C34-L48C47
    await exec('sudo apt-get install libasound2-dev');
  }
}
