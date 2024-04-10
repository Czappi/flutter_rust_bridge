// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.31.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'rust_auto_opaque_twin_moi.dart';
part 'rust_auto_opaque_twin_sse_moi.freezed.dart';

// The type `HelloOneStructTwinSseMoi` is not used by any `pub` functions, thus it is ignored.
// The type `HelloTwoEnumTwinSseMoi` is not used by any `pub` functions, thus it is ignored.

Future<void> rustAutoOpaqueArgOwnTwinSseMoi(
        {required NonCloneSimpleTwinSseMoi arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueArgOwnTwinSseMoi(arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgBorrowTwinSseMoi(
        {required NonCloneSimpleTwinSseMoi arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgBorrowTwinSseMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgMutBorrowTwinSseMoi(
        {required NonCloneSimpleTwinSseMoi arg,
        required int expect,
        required int adder,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgMutBorrowTwinSseMoi(
        arg: arg, expect: expect, adder: adder, hint: hint);

Future<NonCloneSimpleTwinSseMoi> rustAutoOpaqueReturnOwnTwinSseMoi(
        {required int initial, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueReturnOwnTwinSseMoi(initial: initial, hint: hint);

Future<NonCloneSimpleTwinSseMoi> rustAutoOpaqueArgOwnAndReturnOwnTwinSseMoi(
        {required NonCloneSimpleTwinSseMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueArgOwnAndReturnOwnTwinSseMoi(arg: arg, hint: hint);

Future<void> rustAutoOpaqueTwoArgsTwinSseMoi(
        {required NonCloneSimpleTwinSseMoi a,
        required NonCloneSimpleTwinSseMoi b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTwoArgsTwinSseMoi(a: a, b: b, hint: hint);

Future<void> rustAutoOpaqueNormalAndOpaqueArgTwinSseMoi(
        {required NonCloneSimpleTwinSseMoi a,
        required String b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueNormalAndOpaqueArgTwinSseMoi(a: a, b: b, hint: hint);

/// "+" inside the type signature
Future<void> rustAutoOpaquePlusSignArgTwinSseMoi(
        {required BoxMyTraitTwinSseMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaquePlusSignArgTwinSseMoi(arg: arg, hint: hint);

Future<BoxMyTraitTwinSseMoi> rustAutoOpaquePlusSignReturnTwinSseMoi(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaquePlusSignReturnTwinSseMoi(hint: hint);

Future<void> rustAutoOpaqueCallableArgTwinSseMoi(
        {required BoxFnStringString arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueCallableArgTwinSseMoi(arg: arg, hint: hint);

Future<BoxFnStringString> rustAutoOpaqueCallableReturnTwinSseMoi(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueCallableReturnTwinSseMoi(hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgOwnTwinSseMoi(
        {required BoxHelloTraitTwinSseMoi arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgOwnTwinSseMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgBorrowTwinSseMoi(
        {required BoxHelloTraitTwinSseMoi arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgBorrowTwinSseMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgMutBorrowTwinSseMoi(
        {required BoxHelloTraitTwinSseMoi arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgMutBorrowTwinSseMoi(
        arg: arg, expect: expect, hint: hint);

Future<BoxHelloTraitTwinSseMoi> rustAutoOpaqueTraitObjectReturnOwnOneTwinSseMoi(
        {dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTraitObjectReturnOwnOneTwinSseMoi(hint: hint);

Future<BoxHelloTraitTwinSseMoi> rustAutoOpaqueTraitObjectReturnOwnTwoTwinSseMoi(
        {dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTraitObjectReturnOwnTwoTwinSseMoi(hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSseMoi(
        {required StructWithGoodAndOpaqueFieldTwinSseMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSseMoi(
            arg: arg, hint: hint);

Future<StructWithGoodAndOpaqueFieldTwinSseMoi>
    rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSseMoi(
            {dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSseMoi(
                hint: hint);

Future<void> rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSseMoi(
        {required EnumWithGoodAndOpaqueTwinSseMoi arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSseMoi(
        arg: arg, hint: hint);

Future<EnumWithGoodAndOpaqueTwinSseMoi>
    rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinSseMoi(
            {dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinSseMoi(
                hint: hint);

Future<EnumWithGoodAndOpaqueTwinSseMoi>
    rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinSseMoi(
            {dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinSseMoi(
                hint: hint);

Future<void> rustAutoOpaqueDummyTwinSseMoi(
        {required StructWithGoodAndOpaqueFieldWithoutOptionTwinSseMoi a,
        required EnumWithGoodAndOpaqueWithoutOptionTwinSseMoi b,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueDummyTwinSseMoi(a: a, b: b, hint: hint);

Future<void> rustAutoOpaqueEnumArgBorrowTwinSseMoi(
        {required NonCloneSimpleEnumTwinSseMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueEnumArgBorrowTwinSseMoi(arg: arg, hint: hint);

Future<NonCloneSimpleEnumTwinSseMoi> rustAutoOpaqueEnumReturnOwnTwinSseMoi(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueEnumReturnOwnTwinSseMoi(hint: hint);

Future<Stream<NonCloneSimpleTwinSseMoi>> rustAutoOpaqueStreamSinkTwinSseMoi(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueStreamSinkTwinSseMoi(hint: hint);

Future<void> rustAutoOpaqueArgVecOwnTwinSseMoi(
        {required List<NonCloneSimpleTwinSseMoi> arg,
        required List<int> expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgVecOwnTwinSseMoi(
        arg: arg, expect: expect, hint: hint);

Future<List<NonCloneSimpleTwinSseMoi>> rustAutoOpaqueReturnVecOwnTwinSseMoi(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueReturnVecOwnTwinSseMoi(hint: hint);

Future<void> rustAutoOpaqueExplicitArgTwinSseMoi(
        {required NonCloneSimpleTwinSseMoi arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueExplicitArgTwinSseMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueExplicitStructTwinSseMoi(
        {required StructWithExplicitAutoOpaqueFieldTwinSseMoi arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueExplicitStructTwinSseMoi(arg: arg, hint: hint);

Future<NonCloneSimpleTwinSseMoi> rustAutoOpaqueExplicitReturnTwinSseMoi(
        {required int initial, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueExplicitReturnTwinSseMoi(initial: initial, hint: hint);

Future<(OpaqueOneTwinSseMoi, OpaqueTwoTwinSseMoi)>
    rustAutoOpaqueReturnOpaqueOneAndTwoTwinSseMoi({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueReturnOpaqueOneAndTwoTwinSseMoi(hint: hint);

Future<OpaqueTwoTwinSseMoi> rustAutoOpaqueReturnOpaqueTwoTwinSseMoi(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueReturnOpaqueTwoTwinSseMoi(hint: hint);

Future<int> rustAutoOpaqueBorrowAndMutBorrowTwinSseMoi(
        {required NonCloneSimpleTwinSseMoi borrow,
        required NonCloneSimpleTwinSseMoi mutBorrow,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueBorrowAndMutBorrowTwinSseMoi(
        borrow: borrow, mutBorrow: mutBorrow, hint: hint);

Future<int> rustAutoOpaqueBorrowAndBorrowTwinSseMoi(
        {required NonCloneSimpleTwinSseMoi a,
        required NonCloneSimpleTwinSseMoi b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueBorrowAndBorrowTwinSseMoi(a: a, b: b, hint: hint);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<Box < dyn HelloTraitTwinSseMoi >>>
@sealed
class BoxHelloTraitTwinSseMoi extends RustOpaque {
  BoxHelloTraitTwinSseMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxHelloTraitTwinSseMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_BoxHelloTraitTwinSseMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxHelloTraitTwinSseMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxHelloTraitTwinSseMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<Box < dyn MyTraitTwinSseMoi + Send + Sync >>>
@sealed
class BoxMyTraitTwinSseMoi extends RustOpaque {
  BoxMyTraitTwinSseMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxMyTraitTwinSseMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_BoxMyTraitTwinSseMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxMyTraitTwinSseMoi,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxMyTraitTwinSseMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<EnumWithGoodAndOpaqueWithoutOptionTwinSseMoi>>
@sealed
class EnumWithGoodAndOpaqueWithoutOptionTwinSseMoi extends RustOpaque {
  EnumWithGoodAndOpaqueWithoutOptionTwinSseMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  EnumWithGoodAndOpaqueWithoutOptionTwinSseMoi.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_EnumWithGoodAndOpaqueWithoutOptionTwinSseMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_EnumWithGoodAndOpaqueWithoutOptionTwinSseMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_EnumWithGoodAndOpaqueWithoutOptionTwinSseMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<NonCloneSimpleEnumTwinSseMoi>>
@sealed
class NonCloneSimpleEnumTwinSseMoi extends RustOpaque {
  NonCloneSimpleEnumTwinSseMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  NonCloneSimpleEnumTwinSseMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_NonCloneSimpleEnumTwinSseMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleEnumTwinSseMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleEnumTwinSseMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<NonCloneSimpleTwinSseMoi>>
@sealed
class NonCloneSimpleTwinSseMoi extends RustOpaque {
  NonCloneSimpleTwinSseMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  NonCloneSimpleTwinSseMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_NonCloneSimpleTwinSseMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_NonCloneSimpleTwinSseMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleTwinSseMoiPtr,
  );

  Future<void> instanceMethodArgBorrowTwinSseMoi({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseMoiInstanceMethodArgBorrowTwinSseMoi(
              that: this, hint: hint);

  Future<void> instanceMethodArgMutBorrowTwinSseMoi({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseMoiInstanceMethodArgMutBorrowTwinSseMoi(
              that: this, hint: hint);

  Future<void> instanceMethodArgOwnTwinSseMoi({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseMoiInstanceMethodArgOwnTwinSseMoi(
              that: this, hint: hint);

  Future<int> get instanceMethodGetterTwinSseMoi => RustLib.instance.api
          .nonCloneSimpleTwinSseMoiInstanceMethodGetterTwinSseMoi(
        that: this,
      );

  Future<NonCloneSimpleTwinSseMoi> instanceMethodReturnOwnTwinSseMoi(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseMoiInstanceMethodReturnOwnTwinSseMoi(
              that: this, hint: hint);

  /// named constructor
  static Future<NonCloneSimpleTwinSseMoi> newCustomNameTwinSseMoi(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseMoiNewCustomNameTwinSseMoi(hint: hint);

  /// unnamed constructor
  static Future<NonCloneSimpleTwinSseMoi> newTwinSseMoi({dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinSseMoiNewTwinSseMoi(hint: hint);

  /// constructor with Result
  static Future<NonCloneSimpleTwinSseMoi> newWithResultTwinSseMoi(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseMoiNewWithResultTwinSseMoi(hint: hint);

  static Future<void> staticMethodArgBorrowTwinSseMoi(
          {required NonCloneSimpleTwinSseMoi arg, dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseMoiStaticMethodArgBorrowTwinSseMoi(
              arg: arg, hint: hint);

  static Future<void> staticMethodArgMutBorrowTwinSseMoi(
          {required NonCloneSimpleTwinSseMoi arg, dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseMoiStaticMethodArgMutBorrowTwinSseMoi(
              arg: arg, hint: hint);

  static Future<void> staticMethodArgOwnTwinSseMoi(
          {required NonCloneSimpleTwinSseMoi arg, dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinSseMoiStaticMethodArgOwnTwinSseMoi(
          arg: arg, hint: hint);

  static Future<NonCloneSimpleTwinSseMoi> staticMethodReturnOwnTwinSseMoi(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseMoiStaticMethodReturnOwnTwinSseMoi(hint: hint);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueOneTwinSseMoi>>
@sealed
class OpaqueOneTwinSseMoi extends RustOpaque {
  OpaqueOneTwinSseMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  OpaqueOneTwinSseMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_OpaqueOneTwinSseMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueOneTwinSseMoi,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueOneTwinSseMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueTwoTwinSseMoi>>
@sealed
class OpaqueTwoTwinSseMoi extends RustOpaque {
  OpaqueTwoTwinSseMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  OpaqueTwoTwinSseMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_OpaqueTwoTwinSseMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueTwoTwinSseMoi,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueTwoTwinSseMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<StructWithGoodAndOpaqueFieldWithoutOptionTwinSseMoi>>
@sealed
class StructWithGoodAndOpaqueFieldWithoutOptionTwinSseMoi extends RustOpaque {
  StructWithGoodAndOpaqueFieldWithoutOptionTwinSseMoi.dcoDecode(
      List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  StructWithGoodAndOpaqueFieldWithoutOptionTwinSseMoi.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_StructWithGoodAndOpaqueFieldWithoutOptionTwinSseMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldWithoutOptionTwinSseMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldWithoutOptionTwinSseMoiPtr,
  );
}

@freezed
sealed class EnumWithGoodAndOpaqueTwinSseMoi
    with _$EnumWithGoodAndOpaqueTwinSseMoi {
  const EnumWithGoodAndOpaqueTwinSseMoi._();

  const factory EnumWithGoodAndOpaqueTwinSseMoi.good(
    String field0,
  ) = EnumWithGoodAndOpaqueTwinSseMoi_Good;
  const factory EnumWithGoodAndOpaqueTwinSseMoi.opaque(
    NonCloneSimpleTwinSseMoi field0,
  ) = EnumWithGoodAndOpaqueTwinSseMoi_Opaque;
}

class StructWithExplicitAutoOpaqueFieldTwinSseMoi {
  final NonCloneSimpleTwinSseMoi autoOpaque;
  final int normal;

  const StructWithExplicitAutoOpaqueFieldTwinSseMoi({
    required this.autoOpaque,
    required this.normal,
  });

  @override
  int get hashCode => autoOpaque.hashCode ^ normal.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithExplicitAutoOpaqueFieldTwinSseMoi &&
          runtimeType == other.runtimeType &&
          autoOpaque == other.autoOpaque &&
          normal == other.normal;
}

class StructWithGoodAndOpaqueFieldTwinSseMoi {
  final String good;
  final NonCloneSimpleTwinSseMoi opaque;
  final NonCloneSimpleTwinSseMoi? optionOpaque;

  const StructWithGoodAndOpaqueFieldTwinSseMoi({
    required this.good,
    required this.opaque,
    this.optionOpaque,
  });

  @override
  int get hashCode => good.hashCode ^ opaque.hashCode ^ optionOpaque.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithGoodAndOpaqueFieldTwinSseMoi &&
          runtimeType == other.runtimeType &&
          good == other.good &&
          opaque == other.opaque &&
          optionOpaque == other.optionOpaque;
}
