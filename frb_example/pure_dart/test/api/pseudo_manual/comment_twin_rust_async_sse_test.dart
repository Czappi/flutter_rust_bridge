// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `comment_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/comment_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  // do not care too much about the function results, since we are
  // considering the `code comments` feature here, instead of actual function execution logic.
  test('can call the functions', () async {
    await futurizeVoidTwinRustAsyncSse(
        functionWithCommentsSlashStarStarTwinRustAsyncSse());
    await futurizeVoidTwinRustAsyncSse(
        functionWithCommentsTripleSlashMultiLineTwinRustAsyncSse());
    await futurizeVoidTwinRustAsyncSse(
        functionWithCommentsTripleSlashSingleLineTwinRustAsyncSse());
    await futurizeVoidTwinRustAsyncSse(
        StructWithCommentsTwinRustAsyncSse(fieldWithComments: 42)
            .instanceMethodTwinRustAsyncSse());
    await futurizeVoidTwinRustAsyncSse(
        StructWithCommentsTwinRustAsyncSse.staticMethodTwinRustAsyncSse());
  });
}
