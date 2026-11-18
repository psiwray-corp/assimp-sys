# assimp-sys

Raw FFI bindings for [Assimp](https://github.com/assimp/assimp), generated once
with [bindgen](https://rust-lang.github.io/rust-bindgen/) from the Assimp 6.0.6
C headers and committed. The crate links the system Assimp library; it does not
build or vendor Assimp, and consumers do not need `bindgen` or `libclang`.

- 129 functions, 316 constants, and 46 structs from the Assimp C API.
- Direct `extern "C"` calls; no loader and no function-pointer table.
- Callback typedefs are `Option<unsafe extern "C" fn(...)>`.
- Verbatim struct layouts for `aiScene`, `aiMesh`, `aiNode`, `aiMaterial`,
  `aiAnimation`, `aiTexture`, and the math types.
- `links = "assimp"`: only one crate in a dependency graph may link Assimp.

Rust 1.85+ (edition 2024).

## Requirements

An Assimp shared library at least as new as the bound headers (6.0.6) on the
library search path.

| Platform | Install |
|---|---|
| Arch | `pacman -S assimp` |
| Debian/Ubuntu | `apt install libassimp-dev` |
| Fedora | `dnf install assimp-devel` |
| macOS | `brew install assimp` |
| Windows | Build Assimp with CMake and link its `assimp-vc145-mt` import library |

The headers are not needed at build time. On Windows keep `assimp-vc145-mt.dll`
next to the executable; on Unix make sure `libassimp.so.6`/`libassimp.dylib` is
findable.

## Usage

```toml
[dependencies]
assimp-sys = { path = "../assimp-sys" }
```

```rust
use std::ffi::{CStr, CString};

unsafe {
    let path = CString::new("model.obj").unwrap();
    let scene = assimp_sys::aiImportFile(
        path.as_ptr(),
        assimp_sys::aiProcess_Triangulate,
    );
    if scene.is_null() {
        panic!(
            "import failed: {}",
            CStr::from_ptr(assimp_sys::aiGetErrorString()).to_string_lossy()
        );
    }

    // Inspect (*scene).mMeshes, (*scene).mMaterials, ...

    assimp_sys::aiReleaseImport(scene);
}
```

All calls are `unsafe`. Scenes returned by `aiImportFile*` and
`aiApplyPostProcessing` are owned by Assimp until `aiReleaseImport`; property
stores are released with `aiReleasePropertyStore`, and export blobs with
`aiReleaseExportBlob`.

String-valued configuration keys are NUL-terminated byte arrays:

```rust
let key = assimp_sys::AI_CONFIG_IMPORT_FBX_READ_ALL_GEOMETRY_LAYERS
    .as_ptr()
    .cast();
```

## Build configuration

| Variable | Effect |
|---|---|
| `ASSIMP_LIB_DIR` | Adds a native library search path (Homebrew, custom prefix) |
| `ASSIMP_LIB_NAME` | Overrides the linked library name (MSVC toolsets differ) |
| `ASSIMP_STATIC` | Links `assimp`/`assimp-vc145-mt` statically instead of the shared library |

On MSVC the default library name is `assimp-vc145-mt`, matching Assimp 6.0.6's
official Windows build. Use `ASSIMP_LIB_NAME=assimp-vc143-mt` (or another
toolset suffix) for a differently built library.

Static linking pulls in Assimp's own dependencies (zlib, the C++ runtime, and
so on); providing them is the caller's responsibility.

## Scope

Only the C API is bound: `cimport.h`, `cexport.h`, `cfileio.h`, `version.h`,
`config.h`, `scene.h`, `postprocess.h`, and `commonMetaData.h`, plus the types
and constants they include. The C++ API (`Importer.hpp`, `Exporter.hpp`,
`IOSystem.hpp`, ...) and internal headers (`ai_assert.h`, `qnan.h`,
`DefaultLogger.hpp`, ...) are intentionally absent.

The public `AI_MATKEY_*` macros in `GltfMaterial.h` and the deprecated
`pbrmaterial.h` expand to multiple values or take arguments, so bindgen cannot
represent them as Rust constants; use the C API functions with explicit key,
semantic, and index values instead. `pbrmaterial.h` also emits a deprecation
warning under GCC, so it is not part of the bindgen wrapper.

The bindings assume Assimp's default single-precision build (`ai_real = f32`).
A library built with `ASSIMP_DOUBLE_PRECISION` is ABI-incompatible; the smoke
test rejects it via `aiGetCompileFlags()`.

The smoke test also asserts that the linked runtime is not older than the bound
headers (6.0.6). To target an older Assimp, regenerate the bindings from that
version.

## Regenerating

Maintainers only; requires `bindgen-cli` and `libclang`:

```sh
bindgen bindgen/wrapper.h \
  --allowlist-function 'ai.*' \
  --allowlist-type 'ai.*' \
  --allowlist-var 'AI_.*' \
  --allowlist-var 'ASSIMP_CFLAGS_.*' \
  --no-layout-tests --no-prepend-enum-name \
  -o src/bindings.rs -- -I/usr/include
rustfmt --edition 2024 src/bindings.rs
```

Then restore the Assimp notice at the top of `src/bindings.rs` (copy it from
`LICENSE-ASSIMP`); bindgen does not carry the header's license comment over.
The `bindgen/wrapper.h` input is excluded from the published package.

## License

Crate code is MIT (`LICENSE`). `src/bindings.rs` is derived from Assimp's
BSD-3-Clause C headers, retains their copyright notice, and is distributed
under the BSD-3-Clause license (`LICENSE-ASSIMP`). The crate as a whole is
`MIT AND BSD-3-Clause`.

Assimp itself is not distributed by this crate; linking it is governed by its
own BSD-3-Clause license.

## Releases

Branch, tag, and publishing procedures are in
[docs/release/runbook.md](docs/release/runbook.md). Releases are signed locally
and published to the Forgejo registry.
