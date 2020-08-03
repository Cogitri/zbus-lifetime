#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod auto;

pub use crate::auto::*;

macro_rules! apk_array_drop {
    ($x:ident) => {
        impl Drop for $x {
            fn drop(&mut self) {
                unsafe { apk_array_resize(self.item.as_mut_ptr().cast(), 0, 0) };
            }
        }
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apk_package {
    pub hash_node: apk_hash_node,
    pub __bindgen_anon_1: apk_package__bindgen_ty_1,
    pub name: *mut apk_name,
    pub ipkg: *mut apk_installed_package,
    pub version: *mut apk_blob_t,
    pub arch: *mut apk_blob_t,
    pub license: *mut apk_blob_t,
    pub origin: *mut apk_blob_t,
    pub maintainer: *mut apk_blob_t,
    pub url: *mut ::std::os::raw::c_char,
    pub description: *mut ::std::os::raw::c_char,
    pub commit: *mut ::std::os::raw::c_char,
    pub filename: *mut ::std::os::raw::c_char,
    pub depends: *mut apk_dependency_array,
    pub install_if: *mut apk_dependency_array,
    pub provides: *mut apk_dependency_array,
    pub installed_size: size_t,
    pub size: size_t,
    pub build_time: time_t,
    pub provider_priority: ::std::os::raw::c_ushort,
    pub repos: u32,
    pub marked: u8,
    pub uninstallable: u8,
    pub cached_non_repository: u8,
    pub csum: apk_checksum,
}

apk_array_drop!(apk_string_array);
apk_array_drop!(apk_hash_array);
apk_array_drop!(apk_xattr_array);
apk_array_drop!(apk_provider_array);
apk_array_drop!(apk_dependency_array);
apk_array_drop!(apk_package_array);
apk_array_drop!(apk_name_array);
apk_array_drop!(apk_protected_path_array);
apk_array_drop!(apk_change_array);

impl Drop for apk_changeset {
    fn drop(&mut self) {
        unsafe { apk_array_resize(self.changes.cast(), 0, 0) };
    }
}
