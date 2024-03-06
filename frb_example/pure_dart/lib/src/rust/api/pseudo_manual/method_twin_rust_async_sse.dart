// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.26.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<SumWithTwinRustAsyncSse> getSumStructTwinRustAsyncSse({dynamic hint}) =>
    RustLib.instance.api.getSumStructTwinRustAsyncSse(hint: hint);

Future<SumWithTwinRustAsyncSseArray3> getSumArrayTwinRustAsyncSse(
        {required int a, required int b, required int c, dynamic hint}) =>
    RustLib.instance.api
        .getSumArrayTwinRustAsyncSse(a: a, b: b, c: c, hint: hint);

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<ConstructorOpaqueStructTwinRustAsyncSse>>
@sealed
class ConstructorOpaqueStructTwinRustAsyncSse extends RustOpaque {
  ConstructorOpaqueStructTwinRustAsyncSse.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  ConstructorOpaqueStructTwinRustAsyncSse.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_ConstructorOpaqueStructTwinRustAsyncSse,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_ConstructorOpaqueStructTwinRustAsyncSse,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_ConstructorOpaqueStructTwinRustAsyncSsePtr,
  );

  Future<void> checkTwinRustAsyncSse({dynamic hint}) => RustLib.instance.api
          .constructorOpaqueStructTwinRustAsyncSseCheckTwinRustAsyncSse(
        that: this,
      );

  static Future<ConstructorOpaqueStructTwinRustAsyncSse> newTwinRustAsyncSse(
          {dynamic hint}) =>
      RustLib.instance.api
          .constructorOpaqueStructTwinRustAsyncSseNewTwinRustAsyncSse(
              hint: hint);
}

class ConcatenateWithTwinRustAsyncSse {
  final String a;

  const ConcatenateWithTwinRustAsyncSse({
    required this.a,
  });

  static Future<String> concatenateStaticTwinRustAsyncSse(
          {required String a, required String b, dynamic hint}) =>
      RustLib.instance.api
          .concatenateWithTwinRustAsyncSseConcatenateStaticTwinRustAsyncSse(
              a: a, b: b, hint: hint);

  Future<String> concatenateTwinRustAsyncSse(
          {required String b, dynamic hint}) =>
      RustLib.instance.api
          .concatenateWithTwinRustAsyncSseConcatenateTwinRustAsyncSse(
        that: this,
        b: b,
      );

  static Stream<int> handleSomeStaticStreamSinkSingleArgTwinRustAsyncSse(
          {dynamic hint}) =>
      RustLib.instance.api
          .concatenateWithTwinRustAsyncSseHandleSomeStaticStreamSinkSingleArgTwinRustAsyncSse(
              hint: hint);

  static Stream<
      Log2TwinRustAsyncSse> handleSomeStaticStreamSinkTwinRustAsyncSse(
          {required int key, required int max, dynamic hint}) =>
      RustLib.instance.api
          .concatenateWithTwinRustAsyncSseHandleSomeStaticStreamSinkTwinRustAsyncSse(
              key: key, max: max, hint: hint);

  Stream<int> handleSomeStreamSinkAt1TwinRustAsyncSse({dynamic hint}) =>
      RustLib.instance.api
          .concatenateWithTwinRustAsyncSseHandleSomeStreamSinkAt1TwinRustAsyncSse(
        that: this,
      );

  Stream<Log2TwinRustAsyncSse> handleSomeStreamSinkTwinRustAsyncSse(
          {required int key, required int max, dynamic hint}) =>
      RustLib.instance.api
          .concatenateWithTwinRustAsyncSseHandleSomeStreamSinkTwinRustAsyncSse(
        that: this,
        key: key,
        max: max,
      );

  static Future<ConcatenateWithTwinRustAsyncSse> newTwinRustAsyncSse(
          {required String a, dynamic hint}) =>
      RustLib.instance.api
          .concatenateWithTwinRustAsyncSseNewTwinRustAsyncSse(a: a, hint: hint);

  Future<String> get simpleGetterTwinRustAsyncSse => RustLib.instance.api
          .concatenateWithTwinRustAsyncSseSimpleGetterTwinRustAsyncSse(
        that: this,
      );

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ConcatenateWithTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          a == other.a;
}

class ConstructorTranslatableStructTwinRustAsyncSse {
  final String one;

  const ConstructorTranslatableStructTwinRustAsyncSse({
    required this.one,
  });

  static Future<ConstructorTranslatableStructTwinRustAsyncSse>
      newTwinRustAsyncSse({dynamic hint}) => RustLib.instance.api
          .constructorTranslatableStructTwinRustAsyncSseNewTwinRustAsyncSse(
              hint: hint);

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ConstructorTranslatableStructTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          one == other.one;
}

class Log2TwinRustAsyncSse {
  final int key;
  final String value;

  const Log2TwinRustAsyncSse({
    required this.key,
    required this.value,
  });

  @override
  int get hashCode => key.hashCode ^ value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Log2TwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          key == other.key &&
          value == other.value;
}

class MyCallableTwinRustAsyncSse {
  final String one;

  const MyCallableTwinRustAsyncSse({
    required this.one,
  });

  Future<String> call({required String two, dynamic hint}) =>
      RustLib.instance.api.myCallableTwinRustAsyncSseCall(
        that: this,
        two: two,
      );

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MyCallableTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          one == other.one;
}

class SimpleStructTwinRustAsyncSse {
  final String one;

  const SimpleStructTwinRustAsyncSse({
    required this.one,
  });

  static Future<String> argSelfTwinRustAsyncSse(
          {required SimpleStructTwinRustAsyncSse a,
          required SimpleStructTwinRustAsyncSse b,
          dynamic hint}) =>
      RustLib.instance.api.simpleStructTwinRustAsyncSseArgSelfTwinRustAsyncSse(
          a: a, b: b, hint: hint);

  static Future<SimpleStructTwinRustAsyncSse> returnSelfTwinRustAsyncSse(
          {required String one, dynamic hint}) =>
      RustLib.instance.api
          .simpleStructTwinRustAsyncSseReturnSelfTwinRustAsyncSse(
              one: one, hint: hint);

  static Future<List<String>> vecSelfTwinRustAsyncSse(
          {required List<SimpleStructTwinRustAsyncSse> arg, dynamic hint}) =>
      RustLib.instance.api.simpleStructTwinRustAsyncSseVecSelfTwinRustAsyncSse(
          arg: arg, hint: hint);

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SimpleStructTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          one == other.one;
}

class SumWithTwinRustAsyncSse {
  final int x;

  const SumWithTwinRustAsyncSse({
    required this.x,
  });

  Future<int> sumTwinRustAsyncSse(
          {required int y, required int z, dynamic hint}) =>
      RustLib.instance.api.sumWithTwinRustAsyncSseSumTwinRustAsyncSse(
        that: this,
        y: y,
        z: z,
      );

  @override
  int get hashCode => x.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SumWithTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          x == other.x;
}

class SumWithTwinRustAsyncSseArray3
    extends NonGrowableListView<SumWithTwinRustAsyncSse> {
  static const arraySize = 3;

  @internal
  List<SumWithTwinRustAsyncSse> get inner => _inner;
  final List<SumWithTwinRustAsyncSse> _inner;

  SumWithTwinRustAsyncSseArray3(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  SumWithTwinRustAsyncSseArray3.init(SumWithTwinRustAsyncSse fill)
      : this(List<SumWithTwinRustAsyncSse>.filled(arraySize, fill));
}
