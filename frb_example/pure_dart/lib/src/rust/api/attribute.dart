// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'package:meta/meta.dart' as meta;
part 'attribute.freezed.dart';

Future<void> handleCustomizedStructTwinNormal(
        {required CustomizedTwinNormal val, dynamic hint}) =>
    RustLib.instance.api.handleCustomizedStructTwinNormal(val: val, hint: hint);

Future<UserIdTwinNormal> nextUserIdTwinNormal(
        {required UserIdTwinNormal userId, dynamic hint}) =>
    RustLib.instance.api.nextUserIdTwinNormal(userId: userId, hint: hint);

class CustomizedTwinNormal {
  final String finalField;
  String? nonFinalField;

  CustomizedTwinNormal({
    required this.finalField,
    this.nonFinalField,
  });

  @override
  int get hashCode => finalField.hashCode ^ nonFinalField.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomizedTwinNormal &&
          runtimeType == other.runtimeType &&
          finalField == other.finalField &&
          nonFinalField == other.nonFinalField;
}

/// Example for @freezed and @meta.immutable
@freezed
@meta.immutable
class UserIdTwinNormal with _$UserIdTwinNormal {
  const factory UserIdTwinNormal({
    @Default(0) int value,
  }) = _UserIdTwinNormal;
}
