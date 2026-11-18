//! Raw FFI bindings for [Assimp](https://github.com/assimp/assimp).
//!
//! The bindings are generated once with bindgen from the Assimp 6.0.6 C
//! headers and committed, so downstream builds need neither `bindgen` nor
//! `libclang`. The crate links the system Assimp shared library
//! (`libassimp.so`/`libassimp.dylib`, or the `assimp-vc145-mt` import library
//! on Windows) and exposes the C API verbatim.
//!
//! Only the C API is bound: `cimport.h`, `cexport.h`, `cfileio.h`,
//! `version.h`, `config.h`, `scene.h`, `postprocess.h`, and
//! `commonMetaData.h`, plus the types and constants they include. The C++ API
//! (`Importer.hpp`, `Exporter.hpp`, ...) and internal headers (`ai_assert.h`,
//! `qnan.h`, `DefaultLogger.hpp`, ...) are not bound.
//!
//! The public `AI_MATKEY_*` macros in `GltfMaterial.h` and the deprecated
//! `pbrmaterial.h` expand to multiple values or take arguments, so bindgen
//! cannot represent them as Rust constants; use the C API functions with
//! explicit key, semantic, and index values instead.
//!
//! Build-time overrides:
//!
//! - `ASSIMP_LIB_DIR=/path/to/lib`: add a native library search path, e.g. for
//!   Homebrew or a custom prefix.
//! - `ASSIMP_LIB_NAME=<name>`: override the linked library name, e.g.
//!   `assimp-vc143-mt` or `assimp-vc143-mtd` for an MSVC build made with a
//!   different toolset.
//! - `ASSIMP_STATIC=1`: link the static library instead. Transitive system
//!   dependencies (zlib, the C++ runtime, ...) are the caller's
//!   responsibility.
//!
//! The bindings assume Assimp's default single-precision build
//! (`ai_real = f32`). A library built with `ASSIMP_DOUBLE_PRECISION` is
//! ABI-incompatible; the smoke test rejects it via `aiGetCompileFlags()`.
//!
//! Regenerate after updating the pinned headers:
//!
//! ```text
//! bindgen bindgen/wrapper.h \
//!   --allowlist-function 'ai.*' \
//!   --allowlist-type 'ai.*' \
//!   --allowlist-var 'AI_.*' \
//!   --allowlist-var 'ASSIMP_CFLAGS_.*' \
//!   --no-layout-tests --no-prepend-enum-name \
//!   -o src/bindings.rs -- -I/usr/include
//! rustfmt --edition 2024 src/bindings.rs
//! ```
//!
//! Regeneration drops the header comment; restore the Assimp notice at the top
//! of `src/bindings.rs` (copy it from `LICENSE-ASSIMP`).
//!
//! ## Licensing
//!
//! This crate's own code (build script, tests, this file) is MIT-licensed, see
//! `LICENSE`. `src/bindings.rs` is derived from Assimp's BSD-3-Clause C
//! headers, retains their copyright notice, and is distributed under the
//! BSD-3-Clause license, see `LICENSE-ASSIMP`. `Cargo.toml` therefore declares
//! `MIT AND BSD-3-Clause`.
//!
//! Every function is `unsafe`: callers must respect Assimp's ownership rules
//! (release imported scenes with `aiReleaseImport`, property stores with
//! `aiReleasePropertyStore`, and export blobs with `aiReleaseExportBlob`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(clippy::all)]

include!("bindings.rs");
