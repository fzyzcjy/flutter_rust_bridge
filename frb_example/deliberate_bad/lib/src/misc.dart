class CargoBuildAsanInfo {
  static const kExtraArgs = [
    '-Zbuild-std',
    '--target',
    'x86_64-unknown-linux-gnu'
  ];

  static const kExtraEnv = {
    'RUSTFLAGS': '-Zsanitizer=address',
  };
}
