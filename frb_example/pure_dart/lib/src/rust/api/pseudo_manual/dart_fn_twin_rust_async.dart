// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.6.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> rustCallDartSimpleTwinRustAsync(
        {required FutureOr<void> Function() callback, dynamic hint}) =>
    RustLib.instance.api
        .rustCallDartSimpleTwinRustAsync(callback: callback, hint: hint);

Future<void> rustCallDartOneArgTwinRustAsync(
        {required FutureOr<void> Function(String) callback, dynamic hint}) =>
    RustLib.instance.api
        .rustCallDartOneArgTwinRustAsync(callback: callback, hint: hint);

Future<void> rustCallDartTwoArgsTwinRustAsync(
        {required FutureOr<void> Function(
                String, DemoStructForRustCallDartTwinRustAsync)
            callback,
        dynamic hint}) =>
    RustLib.instance.api
        .rustCallDartTwoArgsTwinRustAsync(callback: callback, hint: hint);

Future<void> rustCallDartReturnTwinRustAsync(
        {required FutureOr<String> Function() callback, dynamic hint}) =>
    RustLib.instance.api
        .rustCallDartReturnTwinRustAsync(callback: callback, hint: hint);

Future<void> rustCallDartLoopbackTwinRustAsync(
        {required FutureOr<DemoStructForRustCallDartTwinRustAsync> Function(
                DemoStructForRustCallDartTwinRustAsync)
            callback,
        dynamic hint}) =>
    RustLib.instance.api
        .rustCallDartLoopbackTwinRustAsync(callback: callback, hint: hint);

Future<void> rustCallDartWithDartOpaqueArgTwinRustAsync(
        {required Object input,
        required FutureOr<void> Function(Object) callback,
        dynamic hint}) =>
    RustLib.instance.api.rustCallDartWithDartOpaqueArgTwinRustAsync(
        input: input, callback: callback, hint: hint);

Future<Object> rustCallDartWithDartOpaqueResultTwinRustAsync(
        {required FutureOr<Object> Function() callback, dynamic hint}) =>
    RustLib.instance.api.rustCallDartWithDartOpaqueResultTwinRustAsync(
        callback: callback, hint: hint);

Future<void> rustCallDartMultiTimesTwinRustAsync(
        {required FutureOr<void> Function() callback,
        required int numTimes,
        dynamic hint}) =>
    RustLib.instance.api.rustCallDartMultiTimesTwinRustAsync(
        callback: callback, numTimes: numTimes, hint: hint);

class DemoStructForRustCallDartTwinRustAsync {
  final String name;

  const DemoStructForRustCallDartTwinRustAsync({
    required this.name,
  });

  @override
  int get hashCode => name.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DemoStructForRustCallDartTwinRustAsync &&
          runtimeType == other.runtimeType &&
          name == other.name;
}
