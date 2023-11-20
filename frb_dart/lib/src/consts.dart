import 'package:meta/meta.dart';

/// borrowed from flutter foundation [kIsWeb](https://api.flutter.dev/flutter/foundation/kIsWeb-constant.html),
/// but allows for using it in a Dart context alike
const bool kIsWeb = identical(0, 0.0);

@internal
const kEnableFrbFfiTestTool = bool.fromEnvironment("ENABLE_FRB_FFI_TEST_TOOL");
