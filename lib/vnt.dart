import 'package:ffi/ffi.dart';
import 'package:homophone/generated_bindings.dart';
import 'dart:ffi' as ffi;

void JoinVnt(String secret, String name, String server) {

  final ffifunc = FlutterRustBindings(ffi.DynamicLibrary.open("native.dll"));
  ffifunc.hello_world();

  //var sec= ffi.<ffi.Pointer<ffi.Char>>('hello, world')
  ffifunc.create_vnt(secret.toNativeUtf8().cast(), name.toNativeUtf8().cast(), server.toNativeUtf8().cast());
}

void QuitVnt() {}

String LocalVirsualIp() {
  final ffifunc = FlutterRustBindings(ffi.DynamicLibrary.open("native.dll"));
  final localIp= ffifunc.local_ip();
  return localIp.toString();
}

List<String> Servers() {
  var arrs=  ["127.0.0.1"];
  return arrs;
}
