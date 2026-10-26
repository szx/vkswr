//! Extension properties advertised by this ICD.

use headers::vk_decls::*;

include!(concat!(env!("OUT_DIR"), "/advertised_extensions.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advertised_extension_properties_match_registry() {
        assert_eq!(INSTANCE_EXTENSION_COUNT, 2);
        assert_eq!(DEVICE_EXTENSION_COUNT, 3);
        assert_eq!(INSTANCE_EXTENSION_PROPERTIES[0].specVersion, 25);
        assert_eq!(INSTANCE_EXTENSION_PROPERTIES[1].specVersion, 6);
        assert_eq!(DEVICE_EXTENSION_PROPERTIES[0].specVersion, 70);
        assert_eq!(DEVICE_EXTENSION_PROPERTIES[1].specVersion, 1);
        assert_eq!(DEVICE_EXTENSION_PROPERTIES[2].specVersion, 4);
    }
}
