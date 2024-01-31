pub use sokoban::{FromSlice, NodeAllocatorMap, RedBlackTree};

pub const NONE_SENTINEL: u32 = u32::MAX;
pub const SUCCESS: u32 = 0;
pub const FAILURE: u32 = u32::MAX;

#[macro_export]
macro_rules! red_black_tree_bindings {
    ($key:ty, $value:ty, $size:literal) => {

        ____ci!(tree_type_name = RedBlackTree, $key, $value, $size {

            #[allow(non_snake_case)]
            pub mod tree_type_name {
                use super::*;

                // TODO: this needs to be fixed
                // const SIZE: usize = core::mem::size_of::<tree_type_name>();
                // ____ci!(tree_type_size_name = RedBlackTree, $key, $value, $size, SIZE {
                //     pub const tree_type_size_name: usize = SIZE;
                // });

                #[allow(non_upper_case)]
                pub struct tree_type_name {
                    inner: RedBlackTree<$key, $value, $size>
                }

                // Ensure we can use NONE_SENTINEL as failure
                const _: () = assert!($size < NONE_SENTINEL as usize);

                #[no_mangle]
                pub extern "C" fn initialize(slf: &mut tree_type_name) {
                    slf.inner.initialize()
                }

                #[no_mangle]
                pub unsafe extern "C" fn initialize_in(bytes: *mut u8, len: usize) -> *mut tree_type_name {
                    let byte_slice = unsafe { core::slice::from_raw_parts_mut(bytes, len)};
                    // SAFETY: transparent type
                    RedBlackTree::<$key, $value, $size>::new_from_slice(byte_slice) as *mut _ as *mut tree_type_name
                }

                #[no_mangle]
                pub unsafe extern "C" fn c_insert(slf: &mut tree_type_name, key: $key, value: $value) -> u32 {
                    match slf.inner.insert(key, value) {
                        Some(addr) => addr,
                        None => NONE_SENTINEL,
                    }
                }

                /// Returns 0 if successful, u32::MAX if failure.
                ///
                /// If this fails, the given pointer will point to whatever was there before,
                /// which is potentially uninitialized
                #[no_mangle]
                pub unsafe extern "C" fn c_get(slf: &mut tree_type_name, key: &$key, value: *mut $value) -> u32 {
                    // PERF TODO:
                    //
                    // Because remove_addr is not exposed publicly, we have to choose to either
                    // do two memcpys with remove, or two tree traversals with get/remove.
                    //
                    // Here, we choose to do two tree traversals since we don't know much about the
                    // size of val.
                    match slf.inner.get(key) {
                        Some(val) => {
                            unsafe { *value = *val; }
                            SUCCESS
                        },
                        None => FAILURE
                    }
                }

                /// Returns 0 if successful, u32::MAX if failure.
                ///
                /// If this fails, the given pointer will point to whatever was there before,
                /// which is potentially uninitialized
                #[no_mangle]
                pub unsafe extern "C" fn c_remove(slf: &mut tree_type_name, key: &$key, value: *mut $value) -> u32 {
                    // PERF TODO:
                    //
                    // Because remove_addr is not exposed publicly, we have to choose to either
                    // do two memcpys with remove, or two tree traversals with get/remove.
                    //
                    // Here, we choose to do two tree traversals since we don't know much about the
                    // size of val.
                    match slf.inner.get(key) {
                        Some(val) => {
                            unsafe { *value = *val; }
                            slf.inner.remove(key);
                            SUCCESS
                        },
                        None => FAILURE
                    }
                }
            }

        });

    };
}
