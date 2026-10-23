//! Raw Vulkan FFI declarations and handle representations.
//!
//! [`vk_decls`] is generated from the Vulkan registry at build time. The
//! hand-written types in this module define the handle layout that the Vulkan
//! loader expects from an ICD.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]

pub mod vk_decls;

/// Builds a NUL-terminated `c_char` array at compile time.
///
/// Vulkan uses fixed-size `char` arrays for names such as
/// `VkExtensionProperties::extensionName`.
#[macro_export]
macro_rules! c_char_array {
    ($name:ident, $len:ident, $text:literal) => {
        static $name: [::std::ffi::c_char; $len as usize] = {
            const TEXT: &[u8] = $text.as_bytes();
            let mut array = [0; $len as usize];
            let mut index = 0;
            while index < TEXT.len() && index < array.len() - 1 {
                array[index] = TEXT[index] as ::std::ffi::c_char;
                index += 1;
            }
            array
        };
    };
}
