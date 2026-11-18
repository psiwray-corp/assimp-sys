# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-11-18

### Added

- Checked-in bindgen 0.72.1 bindings for the Assimp 6.0.6 C API: 129
  functions, 316 constants, and 46 structs from `cimport.h`, `cexport.h`,
  `cfileio.h`, `version.h`, `config.h`, `scene.h`, `postprocess.h`, and
  `commonMetaData.h`, so consumers need neither `bindgen` nor `libclang`.
- Dynamic linking against the system Assimp library (`assimp` on Unix and
  MinGW, `assimp-vc145-mt` on MSVC), with `ASSIMP_LIB_DIR`, `ASSIMP_LIB_NAME`,
  and `ASSIMP_STATIC` build overrides and `links = "assimp"` metadata.
- Nullable callback typedefs (`Option<unsafe extern "C" fn(...)>`), verbatim
  struct layouts and constants, and the `AI_CONFIG_*` property-key constants.
- Smoke tests covering dynamic linkage, the linked runtime version, the
  single-precision ABI, and an in-memory PLY import.
- Release branches, signed tags, verification, publishing, and rollback
  scripts and documentation.

### Changed

- Edition 2024, minimum Rust 1.85.
- The crate is `MIT AND BSD-3-Clause`: `src/bindings.rs` derives from Assimp's
  BSD-3-Clause headers and retains their notice.

### Notes

- Only the C API is bound; the C++ API and internal headers are intentionally
  absent.
- The bindings assume Assimp's default single-precision build (`ai_real =
  f32`); double-precision builds are ABI-incompatible.
