# homophone
 homophone

//依赖内网穿透工具项目 https://github.com/lbl8603/vnt,

//
cd ./native
cbindgen src/lib.rs -c cbindgen.toml > target/bindings.h

cd ../
flutter pub add ffi ffigen