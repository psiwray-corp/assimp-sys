use std::ffi::CStr;

// The bindings are generated from Assimp 6.0.6's headers. Assimp's version.h
// exposes functions but no version macros, so the bound version is pinned here.
const BOUND_VERSION: (u32, u32, u32) = (6, 0, 6);

// Compile-time proof of the `-sys` shape: Assimp's callback typedefs are
// nullable, so consumers can pass `None` without inventing fake pointers.
const _: assimp_sys::aiLogStreamCallback = None;
const _: assimp_sys::aiFileOpenProc = None;

#[test]
fn links_against_system_assimp_and_reports_its_version() {
    unsafe {
        let (major, minor, patch) = (
            assimp_sys::aiGetVersionMajor(),
            assimp_sys::aiGetVersionMinor(),
            assimp_sys::aiGetVersionPatch(),
        );

        assert_eq!(
            major, BOUND_VERSION.0,
            "linked Assimp {major}.{minor}.{patch} has a different major version"
        );
        assert!(
            (minor, patch) >= (BOUND_VERSION.1, BOUND_VERSION.2),
            "linked Assimp {major}.{minor}.{patch} is older than the bound headers {BOUND_VERSION:?}"
        );

        // The bindings assume ai_real = f32; a double-precision library is
        // ABI-incompatible.
        assert_eq!(
            assimp_sys::aiGetCompileFlags() & assimp_sys::ASSIMP_CFLAGS_DOUBLE_SUPPORT,
            0,
            "double-precision Assimp builds are ABI-incompatible with these bindings"
        );
    }
}

#[test]
fn imports_ply_from_memory() {
    // A self-contained ASCII PLY triangle; no external material or texture
    // files are needed.
    const PLY: &[u8] = br"ply
format ascii 1.0
element vertex 3
property float x
property float y
property float z
element face 1
property list uchar int vertex_indices
end_header
0 0 0
1 0 0
0 1 0
3 0 1 2
";

    unsafe {
        let scene = assimp_sys::aiImportFileFromMemory(
            PLY.as_ptr().cast(),
            PLY.len() as u32,
            assimp_sys::aiProcess_Triangulate,
            c".ply".as_ptr(),
        );
        if scene.is_null() {
            let error = assimp_sys::aiGetErrorString();
            let message = if error.is_null() {
                "unknown error".to_owned()
            } else {
                CStr::from_ptr(error).to_string_lossy().into_owned()
            };
            panic!("Assimp failed to import the in-memory PLY: {message}");
        }

        assert_eq!((*scene).mNumMeshes, 1);
        let mesh = *(*scene).mMeshes;
        assert_eq!((*mesh).mNumVertices, 3);
        assert_eq!((*mesh).mNumFaces, 1);

        assimp_sys::aiReleaseImport(scene);
    }
}

#[test]
fn metadata_keys_are_bound() {
    assert_eq!(
        assimp_sys::AI_METADATA_SOURCE_FORMAT,
        b"SourceAsset_Format\0"
    );
    assert_eq!(
        assimp_sys::AI_METADATA_SOURCE_GENERATOR,
        b"SourceAsset_Generator\0"
    );
}
