// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.31.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../auxiliary/sample_types.dart';
import '../../frb_generated.dart';
import 'enumeration_twin_rust_async_sse.dart';
import 'misc_example_twin_rust_async_sse.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<Map<int, int>> funcHashMapI32I32TwinRustAsyncSse(
        {required Map<int, int> arg, dynamic hint}) =>
    RustLib.instance.api
        .funcHashMapI32I32TwinRustAsyncSse(arg: arg, hint: hint);

Future<Set<int>> funcHashSetI32TwinRustAsyncSse(
        {required Set<int> arg, dynamic hint}) =>
    RustLib.instance.api.funcHashSetI32TwinRustAsyncSse(arg: arg, hint: hint);

Future<Map<String, String>> funcHashMapStringStringTwinRustAsyncSse(
        {required Map<String, String> arg, dynamic hint}) =>
    RustLib.instance.api
        .funcHashMapStringStringTwinRustAsyncSse(arg: arg, hint: hint);

Future<Set<String>> funcHashSetStringTwinRustAsyncSse(
        {required Set<String> arg, dynamic hint}) =>
    RustLib.instance.api
        .funcHashSetStringTwinRustAsyncSse(arg: arg, hint: hint);

Future<Map<String, Uint8List>> funcHashMapStringBytesTwinRustAsyncSse(
        {required Map<String, Uint8List> arg, dynamic hint}) =>
    RustLib.instance.api
        .funcHashMapStringBytesTwinRustAsyncSse(arg: arg, hint: hint);

Future<Map<String, MySize>> funcHashMapStringStructTwinRustAsyncSse(
        {required Map<String, MySize> arg, dynamic hint}) =>
    RustLib.instance.api
        .funcHashMapStringStructTwinRustAsyncSse(arg: arg, hint: hint);

Future<Map<String, EnumSimpleTwinRustAsyncSse>>
    funcHashMapStringSimpleEnumTwinRustAsyncSse(
            {required Map<String, EnumSimpleTwinRustAsyncSse> arg,
            dynamic hint}) =>
        RustLib.instance.api
            .funcHashMapStringSimpleEnumTwinRustAsyncSse(arg: arg, hint: hint);

Future<Map<String, KitchenSinkTwinRustAsyncSse>>
    funcHashMapStringComplexEnumTwinRustAsyncSse(
            {required Map<String, KitchenSinkTwinRustAsyncSse> arg,
            dynamic hint}) =>
        RustLib.instance.api
            .funcHashMapStringComplexEnumTwinRustAsyncSse(arg: arg, hint: hint);
