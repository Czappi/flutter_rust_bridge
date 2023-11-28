// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<UuidValue> handleUuidTwinNormal({required UuidValue id, dynamic hint}) =>
    RustLib.instance.api.handleUuidTwinNormal(id: id, hint: hint);

Future<List<UuidValue>> handleUuidsTwinNormal(
        {required List<UuidValue> ids, dynamic hint}) =>
    RustLib.instance.api.handleUuidsTwinNormal(ids: ids, hint: hint);

Future<FeatureUuidTwinNormal> handleNestedUuidsTwinNormal(
        {required FeatureUuidTwinNormal ids, dynamic hint}) =>
    RustLib.instance.api.handleNestedUuidsTwinNormal(ids: ids, hint: hint);

class FeatureUuidTwinNormal {
  final UuidValue one;
  final List<UuidValue> many;

  const FeatureUuidTwinNormal({
    required this.one,
    required this.many,
  });

  @override
  int get hashCode => one.hashCode ^ many.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureUuidTwinNormal &&
          runtimeType == other.runtimeType &&
          one == other.one &&
          many == other.many;
}
