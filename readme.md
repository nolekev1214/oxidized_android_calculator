# A Shitty Android Calculator App using Rust
This app is a simple calculator that has two text input fields and four buttons representing the different operators. Data from the text fields is shuttled into a rust backend which uses message passing to send the operands to the parser. 

## Compilation

### Java
IDK, just use Android Studio and hit build

### Rust
Navigate to `app\src\main\rust` and use the command `cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi -t x86_64-linux-android -t i686-linux-android -o./../jniLibs/ build`


# Project Status
Completed