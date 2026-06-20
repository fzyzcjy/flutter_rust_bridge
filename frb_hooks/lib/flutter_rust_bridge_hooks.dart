import 'package:hooks/hooks.dart' as hooks;

export 'src/native_assets_builder.dart';

/// Runs a Native Assets build hook.
Future<void> build(
  List<String> args,
  Future<void> Function(BuildInput input, BuildOutputBuilder output) builder,
) => hooks.build(args, builder);

/// Decides how a produced native asset is routed.
typedef AssetRouting = hooks.AssetRouting;

/// Input data provided to a build hook.
typedef BuildInput = hooks.BuildInput;

/// Creates build hook input values for tests.
typedef BuildInputBuilder = hooks.BuildInputBuilder;

/// Writes assets and metadata produced by a build hook.
typedef BuildOutputBuilder = hooks.BuildOutputBuilder;

/// Base interface for reusable build hook implementations.
typedef Builder = hooks.Builder;

/// Routes a produced native asset to the final application bundle.
typedef ToAppBundle = hooks.ToAppBundle;
