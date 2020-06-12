![{{crate_name}} logo](https://github.com/libcala/{{crate_name}}/blob/master/res/logo.svg)
#### TAGLINE-REPLACE
[![crates.io](https://img.shields.io/crates/v/{{crate_name}}.svg)](https://crates.io/crates/{{crate_name}})
[![docs.rs](https://docs.rs/{{crate_name}}/badge.svg)](https://docs.rs/{{crate_name}})
[![tests](https://github.com/libcala/{{crate_name}}/workflows/tests/badge.svg)](https://github.com/libcala/{{crate_name}}/actions?query=workflow%3Atests)
[![changelog](https://img.shields.io/badge/changelog-green.svg)]()

[Getting Started](https://docs.rs/{{crate_name}}#getting-started) |
[Changelog](https://github.com/libcala/{{crate_name}}/blob/master/CHANGELOG.md) |
[Cala Blog](https://libcala.github.io#blog) |
[GitHub](https://github.com/libcala/{{crate_name}})

# {{crate_name}}
Template to use for libcala libraries: `cargo generate --git https://github.com/libcala/lib_template`

### Supports
 - Doing this cool thing

## Targets
Targets that are marked are currently *supported*, the rest are *planned*.
Targets in **bold** are preferred for that platform.  Targets not listed are
(mostly) similar to the ones listed, so bug reports and PRs can still be opened
for them.  This list is based on how
[cargo-cala](https://github.com/libcala/cargo-cala) builds distribution
packages, so it may seem a little weird.

### Android
 - [ ] APK / AAB (Android App Bundle)
   - [ ] **aarch64-linux-android** (APK: `/lib/arm64-v8a/`)
   - [ ] armv7-linux-androideabi, **thumbv7neon-linux-androideabi** (APK:
     `/lib/armeabi-v7a/`)
   - [ ] **i686-linux-android** (APK: `/lib/x86/`)
   - [ ] **x86_64-linux-android** (APK: `/lib/x86_64/`)

### BSD Variants
 - [ ] **i686-unknown-freebsd** (32-bit FreeBSD App)
 - [ ] **x86_64-unknown-freebsd** (64-bit FreeBSD App)
 - [ ] **x86_64-unknown-netbsd** (64-bit NetBSD App)

### Fuchsia
 - [ ] **aarch64-fuchsia** (Fuchsia for ARM App)
 - [ ] **x86_64-fuchsia** (Fuchsia for X86 App)

### iOS
 - [ ] iOS App
   - [ ] **aarch64-apple-ios**
   - [ ] **x86_64-apple-ios**

### Linux Variants
 - [ ] Flatpak
   - [ ] **x86_64-unknown-linux-gnu** (arch: /x86_64/)
   - [ ] **i586-unknown-linux-gnu** (arch: /i386/)
   - [ ] **aarch64-unknown-linux-gnu** (arch: /aarch64/, Raspberry Pi 4)
   - [ ] armv7-unknown-linux-gnueabihf, **thumbv7neon-unknown-linux-gnueabihf**
     (arch: /arm/, Raspberry Pi 2-3)
 - [ ] **arm-unknown-linux-gnueabihf** (Raspberry Pi Zero W Program)
 - [ ] **riscv64gc-unknown-linux-gnu** (Risc-V Linux Program)

### MacOS
 - [ ] **x86_64-apple-darwin** (MacOS App)

### Redox
 - [ ] **x86_64-unknown-redox** (Redox App)

### Web
 - [ ] Static Web App
   - [ ] wasm32-unknown-emscripten, **wasm32-unknown-unknown**
   - [ ] **asmjs-unknown-emscripten** (fallback)
 - [ ] **wasm32-wasi** (WASI App)

### Windows
 - [ ] i586-pc-windows-msvc, i686-pc-windows-msvc, **i686-pc-windows-gnu**
   (32-bit Windows App),
 - [ ] **x86_64-pc-windows-gnu**, x86\_64-pc-windows-msvc (64-bit Windows App)

## License
Licensed under either of
 - Apache License, Version 2.0,
   ([LICENSE-APACHE](https://github.com/libcala/{{crate_name}}/blob/master/LICENSE-APACHE) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
 - Zlib License,
   ([LICENSE-ZLIB](https://github.com/libcala/{{crate_name}}/blob/master/LICENSE-ZLIB) or
   [https://opensource.org/licenses/Zlib](https://opensource.org/licenses/Zlib))

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Contributors are always welcome (thank you for being interested!), whether it
be a bug report, bug fix, feature request, feature implementation or whatever.
Don't be shy about getting involved.  I always make time to fix bugs, so usually
a patched version of the library will be out a few days after a report.
Features requests will not complete as fast.  If you have any questions, design
critques, or want me to find you something to work on based on your skill level,
you can email me at [jeronlau@plopgrizzly.com](mailto:jeronlau@plopgrizzly.com).
Otherwise,
[here's a link to the issues on GitHub](https://github.com/libcala/{{crate_name}}/issues).
Before contributing, check out the
[contribution guidelines](https://github.com/libcala/{{crate_name}}/blob/master/CONTRIBUTING.md),
and, as always, make sure to follow the
[code of conduct](https://github.com/libcala/{{crate_name}}/blob/master/CODE_OF_CONDUCT.md).
