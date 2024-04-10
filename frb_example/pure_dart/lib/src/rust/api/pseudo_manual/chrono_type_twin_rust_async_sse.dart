// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.31.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<DateTime> datetimeUtcTwinRustAsyncSse(
        {required DateTime d, dynamic hint}) =>
    RustLib.instance.api.datetimeUtcTwinRustAsyncSse(d: d, hint: hint);

Future<DateTime> datetimeLocalTwinRustAsyncSse(
        {required DateTime d, dynamic hint}) =>
    RustLib.instance.api.datetimeLocalTwinRustAsyncSse(d: d, hint: hint);

Future<DateTime> naivedatetimeTwinRustAsyncSse(
        {required DateTime d, dynamic hint}) =>
    RustLib.instance.api.naivedatetimeTwinRustAsyncSse(d: d, hint: hint);

Future<DateTime?> optionalEmptyDatetimeUtcTwinRustAsyncSse(
        {DateTime? d, dynamic hint}) =>
    RustLib.instance.api
        .optionalEmptyDatetimeUtcTwinRustAsyncSse(d: d, hint: hint);

Future<Duration> durationTwinRustAsyncSse(
        {required Duration d, dynamic hint}) =>
    RustLib.instance.api.durationTwinRustAsyncSse(d: d, hint: hint);

Future<List<Duration>> handleTimestampsTwinRustAsyncSse(
        {required List<DateTime> timestamps,
        required DateTime epoch,
        dynamic hint}) =>
    RustLib.instance.api.handleTimestampsTwinRustAsyncSse(
        timestamps: timestamps, epoch: epoch, hint: hint);

Future<List<DateTime>> handleDurationsTwinRustAsyncSse(
        {required List<Duration> durations,
        required DateTime since,
        dynamic hint}) =>
    RustLib.instance.api.handleDurationsTwinRustAsyncSse(
        durations: durations, since: since, hint: hint);

Future<TestChronoTwinRustAsyncSse> testChronoTwinRustAsyncSse({dynamic hint}) =>
    RustLib.instance.api.testChronoTwinRustAsyncSse(hint: hint);

Future<TestChronoTwinRustAsyncSse> testPreciseChronoTwinRustAsyncSse(
        {dynamic hint}) =>
    RustLib.instance.api.testPreciseChronoTwinRustAsyncSse(hint: hint);

Future<Duration> howLongDoesItTakeTwinRustAsyncSse(
        {required FeatureChronoTwinRustAsyncSse mine, dynamic hint}) =>
    RustLib.instance.api
        .howLongDoesItTakeTwinRustAsyncSse(mine: mine, hint: hint);

class FeatureChronoTwinRustAsyncSse {
  final DateTime utc;
  final DateTime local;
  final Duration duration;
  final DateTime naive;

  const FeatureChronoTwinRustAsyncSse({
    required this.utc,
    required this.local,
    required this.duration,
    required this.naive,
  });

  @override
  int get hashCode =>
      utc.hashCode ^ local.hashCode ^ duration.hashCode ^ naive.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureChronoTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          utc == other.utc &&
          local == other.local &&
          duration == other.duration &&
          naive == other.naive;
}

class TestChronoTwinRustAsyncSse {
  final DateTime? dt;
  final DateTime? dt2;
  final Duration? du;

  const TestChronoTwinRustAsyncSse({
    this.dt,
    this.dt2,
    this.du,
  });

  @override
  int get hashCode => dt.hashCode ^ dt2.hashCode ^ du.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TestChronoTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          dt == other.dt &&
          dt2 == other.dt2 &&
          du == other.du;
}
