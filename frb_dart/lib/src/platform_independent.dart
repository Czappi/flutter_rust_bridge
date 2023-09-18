import 'package:meta/meta.dart';
import '../flutter_rust_bridge.dart';

/// Base class for various kinds of tasks.
/// Note: Normally you do not manually create instances of this task (or its brothers), but instead
/// it is generated automatically by the codegen.
@immutable
abstract class FlutterRustBridgeBaseTask {
  /// Metadata that does not change across different method calls.
  final FlutterRustBridgeTaskConstMeta constMeta;

  /// Arguments to be passed into the function call.
  final List<dynamic> argValues;

  /// Transparent hint given by the caller of the method
  final dynamic hint;

  /// Create a new task.
  const FlutterRustBridgeBaseTask({
    required this.constMeta,
    required this.argValues,
    required this.hint,
  });

  /// Name usually used for logging or debugging
  String get debugName => constMeta.debugName;

  /// Arguments to be passed into the function call, provided in the format of a series of [MapEntry]s
  Iterable<MapEntry<String, dynamic>> get argMapEntries sync* {
    for (var i = 0; i < constMeta.argNames.length; ++i) {
      yield MapEntry(constMeta.argNames[i], argValues[i]);
    }
  }

  /// Arguments to be passed into the function call, provided in the format of a [Map]
  Map<String, dynamic> get argMap => Map.fromEntries(argMapEntries);
}

/// Metadata that does not change across different method calls. Thus it is made `const` to save memory and speed up
@immutable
class FlutterRustBridgeTaskConstMeta {
  /// Used for debugging purposes only.
  final String debugName;

  /// A list of arguments to the task.
  final List<String> argNames;

  /// Create a new task metadata.
  const FlutterRustBridgeTaskConstMeta({
    required this.debugName,
    required this.argNames,
  });

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FlutterRustBridgeTaskConstMeta &&
          runtimeType == other.runtimeType &&
          debugName == other.debugName &&
          _listEquals(argNames, other.argNames);

  @override
  int get hashCode => debugName.hashCode ^ Object.hashAll(argNames);

  @override
  String toString() =>
      'FlutterRustBridgeTaskConstMeta{debugName: $debugName, argNames: $argNames}';
}

/// An exception that is generated by Rust code.
@immutable
class FfiException {
  /// The error code of the exception.
  final String code;

  /// The error message of the exception.
  final String message;

  /// The error details of the exception. Optional.
  final Object? details;

  const FfiException(this.code, this.message, [this.details]);

  @override
  String toString() => 'FfiException($code, $message, $details)';
}

/// Exception when timeout happens using [FlutterRustBridgeTimeoutMixin]
@immutable
class FlutterRustBridgeTimeoutException {
  /// The duration to trigger timeout
  final Duration duration;

  /// debugName of the task, usually the ffi function name
  final String debugName;

  /// The stack trace of the error
  final StackTrace stackTrace;

  const FlutterRustBridgeTimeoutException(
      this.duration, this.debugName, this.stackTrace);

  @override
  String toString() =>
      'FlutterRustBridgeTimeoutException(debugName=$debugName, duration=$duration, stackTrace=$stackTrace)';
}

bool _listEquals<T>(List<T>? a, List<T>? b) {
  if (a == null) return b == null;
  if (b == null || a.length != b.length) return false;
  if (identical(a, b)) return true;
  for (var index = 0; index < a.length; index += 1) {
    if (a[index] != b[index]) return false;
  }
  return true;
}
