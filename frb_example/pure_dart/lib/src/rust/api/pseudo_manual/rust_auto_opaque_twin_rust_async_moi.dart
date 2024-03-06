// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.26.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'rust_auto_opaque_twin_moi.dart';
part 'rust_auto_opaque_twin_rust_async_moi.freezed.dart';

// The type `HelloOneStructTwinRustAsyncMoi` is not used by any `pub` functions, thus it is ignored.
// The type `HelloTwoEnumTwinRustAsyncMoi` is not used by any `pub` functions, thus it is ignored.

Future<void> rustAutoOpaqueArgOwnTwinRustAsyncMoi(
        {required NonCloneSimpleTwinRustAsyncMoi arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgOwnTwinRustAsyncMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgBorrowTwinRustAsyncMoi(
        {required NonCloneSimpleTwinRustAsyncMoi arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgBorrowTwinRustAsyncMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgMutBorrowTwinRustAsyncMoi(
        {required NonCloneSimpleTwinRustAsyncMoi arg,
        required int expect,
        required int adder,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgMutBorrowTwinRustAsyncMoi(
        arg: arg, expect: expect, adder: adder, hint: hint);

Future<NonCloneSimpleTwinRustAsyncMoi> rustAutoOpaqueReturnOwnTwinRustAsyncMoi(
        {required int initial, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: initial, hint: hint);

Future<NonCloneSimpleTwinRustAsyncMoi>
    rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsyncMoi(
            {required NonCloneSimpleTwinRustAsyncMoi arg, dynamic hint}) =>
        RustLib.instance.api.rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsyncMoi(
            arg: arg, hint: hint);

Future<void> rustAutoOpaqueTwoArgsTwinRustAsyncMoi(
        {required NonCloneSimpleTwinRustAsyncMoi a,
        required NonCloneSimpleTwinRustAsyncMoi b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTwoArgsTwinRustAsyncMoi(a: a, b: b, hint: hint);

Future<void> rustAutoOpaqueNormalAndOpaqueArgTwinRustAsyncMoi(
        {required NonCloneSimpleTwinRustAsyncMoi a,
        required String b,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueNormalAndOpaqueArgTwinRustAsyncMoi(
        a: a, b: b, hint: hint);

/// "+" inside the type signature
Future<void> rustAutoOpaquePlusSignArgTwinRustAsyncMoi(
        {required BoxMyTraitTwinRustAsyncMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaquePlusSignArgTwinRustAsyncMoi(arg: arg, hint: hint);

Future<BoxMyTraitTwinRustAsyncMoi> rustAutoOpaquePlusSignReturnTwinRustAsyncMoi(
        {dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaquePlusSignReturnTwinRustAsyncMoi(hint: hint);

Future<void> rustAutoOpaqueCallableArgTwinRustAsyncMoi(
        {required BoxFnStringString arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueCallableArgTwinRustAsyncMoi(arg: arg, hint: hint);

Future<BoxFnStringString> rustAutoOpaqueCallableReturnTwinRustAsyncMoi(
        {dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueCallableReturnTwinRustAsyncMoi(hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgOwnTwinRustAsyncMoi(
        {required BoxHelloTraitTwinRustAsyncMoi arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgOwnTwinRustAsyncMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgBorrowTwinRustAsyncMoi(
        {required BoxHelloTraitTwinRustAsyncMoi arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgBorrowTwinRustAsyncMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsyncMoi(
        {required BoxHelloTraitTwinRustAsyncMoi arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsyncMoi(
        arg: arg, expect: expect, hint: hint);

Future<BoxHelloTraitTwinRustAsyncMoi>
    rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsyncMoi({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsyncMoi(hint: hint);

Future<BoxHelloTraitTwinRustAsyncMoi>
    rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsyncMoi({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsyncMoi(hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsyncMoi(
        {required StructWithGoodAndOpaqueFieldTwinRustAsyncMoi arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsyncMoi(
            arg: arg, hint: hint);

Future<StructWithGoodAndOpaqueFieldTwinRustAsyncMoi>
    rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsyncMoi(
            {dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsyncMoi(
                hint: hint);

Future<void> rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsyncMoi(
        {required EnumWithGoodAndOpaqueTwinRustAsyncMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsyncMoi(
            arg: arg, hint: hint);

Future<EnumWithGoodAndOpaqueTwinRustAsyncMoi>
    rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinRustAsyncMoi(
            {dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinRustAsyncMoi(
                hint: hint);

Future<EnumWithGoodAndOpaqueTwinRustAsyncMoi>
    rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinRustAsyncMoi(
            {dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinRustAsyncMoi(
                hint: hint);

Future<void> rustAutoOpaqueDummyTwinRustAsyncMoi(
        {required StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncMoi a,
        required EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncMoi b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueDummyTwinRustAsyncMoi(a: a, b: b, hint: hint);

Future<void> rustAutoOpaqueEnumArgBorrowTwinRustAsyncMoi(
        {required NonCloneSimpleEnumTwinRustAsyncMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueEnumArgBorrowTwinRustAsyncMoi(arg: arg, hint: hint);

Future<NonCloneSimpleEnumTwinRustAsyncMoi>
    rustAutoOpaqueEnumReturnOwnTwinRustAsyncMoi({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueEnumReturnOwnTwinRustAsyncMoi(hint: hint);

Stream<NonCloneSimpleTwinRustAsyncMoi> rustAutoOpaqueStreamSinkTwinRustAsyncMoi(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueStreamSinkTwinRustAsyncMoi(hint: hint);

Future<void> rustAutoOpaqueArgVecOwnTwinRustAsyncMoi(
        {required List<NonCloneSimpleTwinRustAsyncMoi> arg,
        required List<int> expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgVecOwnTwinRustAsyncMoi(
        arg: arg, expect: expect, hint: hint);

Future<List<NonCloneSimpleTwinRustAsyncMoi>>
    rustAutoOpaqueReturnVecOwnTwinRustAsyncMoi({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueReturnVecOwnTwinRustAsyncMoi(hint: hint);

Future<void> rustAutoOpaqueExplicitArgTwinRustAsyncMoi(
        {required NonCloneSimpleTwinRustAsyncMoi arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueExplicitArgTwinRustAsyncMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueExplicitStructTwinRustAsyncMoi(
        {required StructWithExplicitAutoOpaqueFieldTwinRustAsyncMoi arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueExplicitStructTwinRustAsyncMoi(arg: arg, hint: hint);

Future<NonCloneSimpleTwinRustAsyncMoi>
    rustAutoOpaqueExplicitReturnTwinRustAsyncMoi(
            {required int initial, dynamic hint}) =>
        RustLib.instance.api.rustAutoOpaqueExplicitReturnTwinRustAsyncMoi(
            initial: initial, hint: hint);

Future<(OpaqueOneTwinRustAsyncMoi, OpaqueTwoTwinRustAsyncMoi)>
    rustAutoOpaqueReturnOpaqueOneAndTwoTwinRustAsyncMoi({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueReturnOpaqueOneAndTwoTwinRustAsyncMoi(hint: hint);

Future<OpaqueTwoTwinRustAsyncMoi> rustAutoOpaqueReturnOpaqueTwoTwinRustAsyncMoi(
        {dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueReturnOpaqueTwoTwinRustAsyncMoi(hint: hint);

Future<int> rustAutoOpaqueBorrowAndMutBorrowTwinRustAsyncMoi(
        {required NonCloneSimpleTwinRustAsyncMoi borrow,
        required NonCloneSimpleTwinRustAsyncMoi mutBorrow,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueBorrowAndMutBorrowTwinRustAsyncMoi(
        borrow: borrow, mutBorrow: mutBorrow, hint: hint);

Future<int> rustAutoOpaqueBorrowAndBorrowTwinRustAsyncMoi(
        {required NonCloneSimpleTwinRustAsyncMoi a,
        required NonCloneSimpleTwinRustAsyncMoi b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueBorrowAndBorrowTwinRustAsyncMoi(a: a, b: b, hint: hint);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<Box < dyn HelloTraitTwinRustAsyncMoi >>>
@sealed
class BoxHelloTraitTwinRustAsyncMoi extends RustOpaque {
  BoxHelloTraitTwinRustAsyncMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxHelloTraitTwinRustAsyncMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_BoxHelloTraitTwinRustAsyncMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxHelloTraitTwinRustAsyncMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxHelloTraitTwinRustAsyncMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<Box < dyn MyTraitTwinRustAsyncMoi + Send + Sync >>>
@sealed
class BoxMyTraitTwinRustAsyncMoi extends RustOpaque {
  BoxMyTraitTwinRustAsyncMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxMyTraitTwinRustAsyncMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_BoxMyTraitTwinRustAsyncMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxMyTraitTwinRustAsyncMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxMyTraitTwinRustAsyncMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncMoi>>
@sealed
class EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncMoi extends RustOpaque {
  EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncMoi.dcoDecode(
      List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncMoi.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<NonCloneSimpleEnumTwinRustAsyncMoi>>
@sealed
class NonCloneSimpleEnumTwinRustAsyncMoi extends RustOpaque {
  NonCloneSimpleEnumTwinRustAsyncMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  NonCloneSimpleEnumTwinRustAsyncMoi.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_NonCloneSimpleEnumTwinRustAsyncMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleEnumTwinRustAsyncMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleEnumTwinRustAsyncMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<NonCloneSimpleTwinRustAsyncMoi>>
@sealed
class NonCloneSimpleTwinRustAsyncMoi extends RustOpaque {
  NonCloneSimpleTwinRustAsyncMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  NonCloneSimpleTwinRustAsyncMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_NonCloneSimpleTwinRustAsyncMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleTwinRustAsyncMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleTwinRustAsyncMoiPtr,
  );

  Future<void> instanceMethodArgBorrowTwinRustAsyncMoi({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiInstanceMethodArgBorrowTwinRustAsyncMoi(
        that: this,
      );

  Future<void> instanceMethodArgMutBorrowTwinRustAsyncMoi({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiInstanceMethodArgMutBorrowTwinRustAsyncMoi(
        that: this,
      );

  Future<void> instanceMethodArgOwnTwinRustAsyncMoi({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiInstanceMethodArgOwnTwinRustAsyncMoi(
        that: this,
      );

  Future<int> get instanceMethodGetterTwinRustAsyncMoi => RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiInstanceMethodGetterTwinRustAsyncMoi(
        that: this,
      );

  Future<
      NonCloneSimpleTwinRustAsyncMoi> instanceMethodReturnOwnTwinRustAsyncMoi(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiInstanceMethodReturnOwnTwinRustAsyncMoi(
        that: this,
      );

  /// named constructor
  static Future<NonCloneSimpleTwinRustAsyncMoi> newCustomNameTwinRustAsyncMoi(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiNewCustomNameTwinRustAsyncMoi(
              hint: hint);

  /// unnamed constructor
  static Future<NonCloneSimpleTwinRustAsyncMoi> newTwinRustAsyncMoi(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiNewTwinRustAsyncMoi(hint: hint);

  /// constructor with Result
  static Future<NonCloneSimpleTwinRustAsyncMoi> newWithResultTwinRustAsyncMoi(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiNewWithResultTwinRustAsyncMoi(
              hint: hint);

  static Future<void> staticMethodArgBorrowTwinRustAsyncMoi(
          {required NonCloneSimpleTwinRustAsyncMoi arg, dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiStaticMethodArgBorrowTwinRustAsyncMoi(
              arg: arg, hint: hint);

  static Future<void> staticMethodArgMutBorrowTwinRustAsyncMoi(
          {required NonCloneSimpleTwinRustAsyncMoi arg, dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiStaticMethodArgMutBorrowTwinRustAsyncMoi(
              arg: arg, hint: hint);

  static Future<void> staticMethodArgOwnTwinRustAsyncMoi(
          {required NonCloneSimpleTwinRustAsyncMoi arg, dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncMoiStaticMethodArgOwnTwinRustAsyncMoi(
              arg: arg, hint: hint);

  static Future<NonCloneSimpleTwinRustAsyncMoi>
      staticMethodReturnOwnTwinRustAsyncMoi({dynamic hint}) => RustLib
          .instance.api
          .nonCloneSimpleTwinRustAsyncMoiStaticMethodReturnOwnTwinRustAsyncMoi(
              hint: hint);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueOneTwinRustAsyncMoi>>
@sealed
class OpaqueOneTwinRustAsyncMoi extends RustOpaque {
  OpaqueOneTwinRustAsyncMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  OpaqueOneTwinRustAsyncMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_OpaqueOneTwinRustAsyncMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueOneTwinRustAsyncMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_OpaqueOneTwinRustAsyncMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueTwoTwinRustAsyncMoi>>
@sealed
class OpaqueTwoTwinRustAsyncMoi extends RustOpaque {
  OpaqueTwoTwinRustAsyncMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  OpaqueTwoTwinRustAsyncMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_OpaqueTwoTwinRustAsyncMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueTwoTwinRustAsyncMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_OpaqueTwoTwinRustAsyncMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncMoi>>
@sealed
class StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncMoi
    extends RustOpaque {
  StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncMoi.dcoDecode(
      List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncMoi.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncMoiPtr,
  );
}

@freezed
sealed class EnumWithGoodAndOpaqueTwinRustAsyncMoi
    with _$EnumWithGoodAndOpaqueTwinRustAsyncMoi {
  const factory EnumWithGoodAndOpaqueTwinRustAsyncMoi.good(
    String field0,
  ) = EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good;
  const factory EnumWithGoodAndOpaqueTwinRustAsyncMoi.opaque(
    NonCloneSimpleTwinRustAsyncMoi field0,
  ) = EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque;
}

class StructWithExplicitAutoOpaqueFieldTwinRustAsyncMoi {
  final NonCloneSimpleTwinRustAsyncMoi autoOpaque;
  final int normal;

  const StructWithExplicitAutoOpaqueFieldTwinRustAsyncMoi({
    required this.autoOpaque,
    required this.normal,
  });

  @override
  int get hashCode => autoOpaque.hashCode ^ normal.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithExplicitAutoOpaqueFieldTwinRustAsyncMoi &&
          runtimeType == other.runtimeType &&
          autoOpaque == other.autoOpaque &&
          normal == other.normal;
}

class StructWithGoodAndOpaqueFieldTwinRustAsyncMoi {
  final String good;
  final NonCloneSimpleTwinRustAsyncMoi opaque;

  const StructWithGoodAndOpaqueFieldTwinRustAsyncMoi({
    required this.good,
    required this.opaque,
  });

  @override
  int get hashCode => good.hashCode ^ opaque.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithGoodAndOpaqueFieldTwinRustAsyncMoi &&
          runtimeType == other.runtimeType &&
          good == other.good &&
          opaque == other.opaque;
}
