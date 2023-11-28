// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'dart_opaque.freezed.dart';

Future<String> asyncAcceptDartOpaqueTwinNormal(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .asyncAcceptDartOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<Object> loopBackTwinNormal({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackTwinNormal(opaque: opaque, hint: hint);

Future<Object?> loopBackOptionTwinNormal(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackOptionTwinNormal(opaque: opaque, hint: hint);

Future<ObjectArray1> loopBackArrayTwinNormal(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackArrayTwinNormal(opaque: opaque, hint: hint);

Future<List<Object>> loopBackVecTwinNormal(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackVecTwinNormal(opaque: opaque, hint: hint);

Future<void> loopBackOptionGetTwinNormal({Object? opaque, dynamic hint}) =>
    RustLib.instance.api
        .loopBackOptionGetTwinNormal(opaque: opaque, hint: hint);

Future<void> loopBackArrayGetTwinNormal(
        {required ObjectArray1 opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackArrayGetTwinNormal(opaque: opaque, hint: hint);

Future<void> loopBackVecGetTwinNormal(
        {required List<Object> opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackVecGetTwinNormal(opaque: opaque, hint: hint);

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
Future<void> panicUnwrapDartOpaqueTwinNormal(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .panicUnwrapDartOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<DartOpaqueNestedTwinNormal> createNestedDartOpaqueTwinNormal(
        {required Object opaque1, required Object opaque2, dynamic hint}) =>
    RustLib.instance.api.createNestedDartOpaqueTwinNormal(
        opaque1: opaque1, opaque2: opaque2, hint: hint);

Future<void> getNestedDartOpaqueTwinNormal(
        {required DartOpaqueNestedTwinNormal opaque, dynamic hint}) =>
    RustLib.instance.api
        .getNestedDartOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<EnumDartOpaqueTwinNormal> createEnumDartOpaqueTwinNormal(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .createEnumDartOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<void> getEnumDartOpaqueTwinNormal(
        {required EnumDartOpaqueTwinNormal opaque, dynamic hint}) =>
    RustLib.instance.api
        .getEnumDartOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<void> setStaticDartOpaqueTwinNormal(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .setStaticDartOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<void> dropStaticDartOpaqueTwinNormal({dynamic hint}) =>
    RustLib.instance.api.dropStaticDartOpaqueTwinNormal(hint: hint);

class ObjectArray1 extends NonGrowableListView<Object> {
  static const arraySize = 1;
  ObjectArray1(List<Object> inner)
      : assert(inner.length == arraySize),
        super(inner);
  ObjectArray1.unchecked(List<Object> inner) : super(inner);
  ObjectArray1.init(Object fill) : super(List<Object>.filled(arraySize, fill));
}

class DartOpaqueNestedTwinNormal {
  final Object first;
  final Object second;

  const DartOpaqueNestedTwinNormal({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DartOpaqueNestedTwinNormal &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}

@freezed
sealed class EnumDartOpaqueTwinNormal with _$EnumDartOpaqueTwinNormal {
  const factory EnumDartOpaqueTwinNormal.primitive(
    int field0,
  ) = EnumDartOpaqueTwinNormal_Primitive;
  const factory EnumDartOpaqueTwinNormal.opaque(
    Object field0,
  ) = EnumDartOpaqueTwinNormal_Opaque;
}
