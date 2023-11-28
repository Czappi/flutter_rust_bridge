// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'misc_example.dart';
import 'newtype_pattern.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<double?> handleOptionalReturnTwinNormal(
        {required double left, required double right, dynamic hint}) =>
    RustLib.instance.api
        .handleOptionalReturnTwinNormal(left: left, right: right, hint: hint);

Future<ElementTwinNormal?> handleOptionalStructTwinNormal(
        {String? document, dynamic hint}) =>
    RustLib.instance.api
        .handleOptionalStructTwinNormal(document: document, hint: hint);

Future<ExoticOptionalsTwinNormal?> handleOptionalIncrementTwinNormal(
        {ExoticOptionalsTwinNormal? opt, dynamic hint}) =>
    RustLib.instance.api
        .handleOptionalIncrementTwinNormal(opt: opt, hint: hint);

Future<double> handleIncrementBoxedOptionalTwinNormal(
        {double? opt, dynamic hint}) =>
    RustLib.instance.api
        .handleIncrementBoxedOptionalTwinNormal(opt: opt, hint: hint);

Future<OptVecsTwinNormal> handleVecOfOptsTwinNormal(
        {required OptVecsTwinNormal opt, dynamic hint}) =>
    RustLib.instance.api.handleVecOfOptsTwinNormal(opt: opt, hint: hint);

Future<String> handleOptionBoxArgumentsTwinNormal(
        {int? i8Box,
        int? u8Box,
        int? i32Box,
        int? i64Box,
        double? f64Box,
        bool? boolbox,
        ExoticOptionalsTwinNormal? structbox,
        dynamic hint}) =>
    RustLib.instance.api.handleOptionBoxArgumentsTwinNormal(
        i8Box: i8Box,
        u8Box: u8Box,
        i32Box: i32Box,
        i64Box: i64Box,
        f64Box: f64Box,
        boolbox: boolbox,
        structbox: structbox,
        hint: hint);

String? syncOptionTwinNormal({dynamic hint}) =>
    RustLib.instance.api.syncOptionTwinNormal(hint: hint);

String? syncOptionNullTwinNormal({dynamic hint}) =>
    RustLib.instance.api.syncOptionNullTwinNormal(hint: hint);

class AttributeTwinNormal {
  final String key;
  final String value;

  const AttributeTwinNormal({
    required this.key,
    required this.value,
  });

  @override
  int get hashCode => key.hashCode ^ value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AttributeTwinNormal &&
          runtimeType == other.runtimeType &&
          key == other.key &&
          value == other.value;
}

class ElementTwinNormal {
  final String? tag;
  final String? text;
  final List<AttributeTwinNormal>? attributes;
  final List<ElementTwinNormal>? children;

  const ElementTwinNormal({
    this.tag,
    this.text,
    this.attributes,
    this.children,
  });

  @override
  int get hashCode =>
      tag.hashCode ^ text.hashCode ^ attributes.hashCode ^ children.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ElementTwinNormal &&
          runtimeType == other.runtimeType &&
          tag == other.tag &&
          text == other.text &&
          attributes == other.attributes &&
          children == other.children;
}

class ExoticOptionalsTwinNormal {
  final int? int32;
  final int? int64;
  final double? float64;
  final bool? boolean;
  final Uint8List? zerocopy;
  final Int8List? int8List;
  final Uint8List? uint8List;
  final Int32List? int32List;
  final Float32List? float32List;
  final Float64List? float64List;
  final List<AttributeTwinNormal>? attributes;
  final List<AttributeTwinNormal?> attributesNullable;
  final List<AttributeTwinNormal?>? nullableAttributes;
  final NewTypeIntTwinNormal? newtypeint;

  const ExoticOptionalsTwinNormal({
    this.int32,
    this.int64,
    this.float64,
    this.boolean,
    this.zerocopy,
    this.int8List,
    this.uint8List,
    this.int32List,
    this.float32List,
    this.float64List,
    this.attributes,
    required this.attributesNullable,
    this.nullableAttributes,
    this.newtypeint,
  });

  @override
  int get hashCode =>
      int32.hashCode ^
      int64.hashCode ^
      float64.hashCode ^
      boolean.hashCode ^
      zerocopy.hashCode ^
      int8List.hashCode ^
      uint8List.hashCode ^
      int32List.hashCode ^
      float32List.hashCode ^
      float64List.hashCode ^
      attributes.hashCode ^
      attributesNullable.hashCode ^
      nullableAttributes.hashCode ^
      newtypeint.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ExoticOptionalsTwinNormal &&
          runtimeType == other.runtimeType &&
          int32 == other.int32 &&
          int64 == other.int64 &&
          float64 == other.float64 &&
          boolean == other.boolean &&
          zerocopy == other.zerocopy &&
          int8List == other.int8List &&
          uint8List == other.uint8List &&
          int32List == other.int32List &&
          float32List == other.float32List &&
          float64List == other.float64List &&
          attributes == other.attributes &&
          attributesNullable == other.attributesNullable &&
          nullableAttributes == other.nullableAttributes &&
          newtypeint == other.newtypeint;
}

class OptVecsTwinNormal {
  final List<int?> i32;
  final List<WeekdaysTwinNormal?> enums;
  final List<String?> strings;
  final List<Int32List?> buffers;

  const OptVecsTwinNormal({
    required this.i32,
    required this.enums,
    required this.strings,
    required this.buffers,
  });

  @override
  int get hashCode =>
      i32.hashCode ^ enums.hashCode ^ strings.hashCode ^ buffers.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is OptVecsTwinNormal &&
          runtimeType == other.runtimeType &&
          i32 == other.i32 &&
          enums == other.enums &&
          strings == other.strings &&
          buffers == other.buffers;
}
