use phf::phf_map;
use proc_macro2::{Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{DeError, Reader, Writer};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Eq, Hash, PartialEq, Debug, serde::Deserialize)]
pub struct VkEnumeration {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@type")]
    type_: Option<String>,

    #[serde(rename = "enum")]
    //#[serde(rename = "$value")]
    enumerators: Option<Vec<VkEnumerator>>,
}

impl ToTokens for VkEnumeration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", &self.name);
        if let Some(ref enumerators) = &self.enumerators {
            tokens.extend(quote! {
                #[repr(C)]
                enum #name {
                    #(#enumerators)*
                }
            });

            for enumerator in enumerators {
                if let Some(alias) = enumerator.aliases_to_tokens(&self.name) {
                    tokens.extend(quote! {
                        impl #name {
                            pub const #alias;
                        }
                    });
                }
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, serde::Deserialize)]
struct VkEnumerator {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@type")]
    type_: Option<String>,

    #[serde(rename = "@value")]
    value: Option<String>,

    #[serde(rename = "@alias")]
    alias: Option<String>,
}

impl VkEnumerator {
    fn aliases_to_tokens(&self, enumeration_name: &str) -> Option<TokenStream> {
        if let Some(ref alias) = self.alias {
            let enumeration_name = format_ident!("{}", enumeration_name);
            let name = format_ident!("{}", self.name);
            let alias = format_ident!("{}", alias);
            Some(quote!(#name: #enumeration_name = #enumeration_name::#alias).to_token_stream())
        } else {
            None
        }
    }
}

impl ToTokens for VkEnumerator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(ref alias) = self.alias {
            // Skip, we have to use associated constants (aliases_to_tokens).
        } else if let Some(ref value) = self.value {
            tokens.append(format_ident!("{}", self.name));
            tokens.append(Punct::new('=', Spacing::Alone));
            if let Some(token) = (|| -> Option<Literal> {
                match self.type_.as_deref() {
                    Some("uint32_t") => Some(Literal::u32_suffixed(value.parse().ok()?)),
                    Some("float") => Some(Literal::f32_suffixed(value.parse().ok()?)),
                    Some("uint64_t") => Some(Literal::u64_suffixed(value.parse().ok()?)),
                    None => Some(Literal::isize_unsuffixed(
                        parse_int::parse::<isize>(value).ok()?,
                    )),
                    _ => None,
                }
            })() {
                tokens.append(token)
            }
            tokens.append(Punct::new(',', Spacing::Alone));
        } else {
            tokens.append(format_ident!("{}", self.name));
            tokens.append(Punct::new(',', Spacing::Alone));
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, serde::Deserialize)]
pub struct VkStruct {
    #[serde(rename = "@category")]
    category: String,

    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "member")]
    members: Option<Vec<VkStructMember>>,
}

impl VkStruct {
    pub fn fix(&mut self) {
        let Some(ref mut members) = self.members else { return };
        for member in members.iter_mut() {
            if member.name == "type" {
                member.name = "type_".into();
            };

            let Some(ref mut type_) = member.type_ else { return };
            if let Some(ref mut type2) = member.type2 {
                type_.push_str(type2);
                member.type2 = None;
            }
        }
    }
}

impl ToTokens for VkStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", &self.name);
        if let Some(ref members) = &self.members {
            tokens.extend(quote! {
                #[repr(C)]
                struct #name {
                    #(#members)*
                }
            });
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, serde::Deserialize)]
pub struct VkStructMember {
    #[serde(rename = "type")]
    type_: Option<String>,

    #[serde(rename = "$text")]
    type2: Option<String>,

    #[serde(rename = "name")]
    name: String,
}


impl ToTokens for VkStructMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(format_ident!("{}", self.name));
        tokens.append(Punct::new(':', Spacing::Alone));
        let Some( type_) = self.type_.as_deref() else { unreachable!() };
        let Ok(ffi_type) : Result<TokenStream, _> = C_TYPE_TO_RUST.get(type_).unwrap_or(&type_).parse() else { unreachable!() };
        tokens.extend(ffi_type);
        tokens.append(Punct::new(',', Spacing::Alone));
    }
}

static C_TYPE_TO_RUST: phf::Map<&'static str, &'static str> = phf_map! {
    "uint8_t" => "u8",
    "uint16_t" => "u16",
    "uint32_t" => "u32",
    "uint64_t" => "u64",
    "size_t" => "isize",
    "int32_t" => "i32",
    "int64_t" => "i64",
    "float" => "f32",
    "char" => "std::ffi::c_char",
    "void*" => "std::ffi::c_void",
    "uint32_t[2]" => "[u32;2]",
    "uint32_t[3]" => "[u32;3]",
    "float[2]" => "[f32;2]",
    "float[3][4]" => "[[f32;3];4]",
    //
    "VkOffset3D[2]" => "[VkOffset3D; 2]",
    "VkDrmFormatModifierPropertiesEXT*" => "*mut VkDrmFormatModifierPropertiesEXT",
    "VkDrmFormatModifierProperties2EXT*" => "*mut VkDrmFormatModifierProperties2EXT",
    "VkPresentModeKHR*" => "*mut VkPresentModeKHR",
    // TODO: How do repr(C) bitfields work?
    "uint32_t:24" => "[u8;3]",
    "uint32_t:8" => "[u8;1]",
    "VkGeometryInstanceFlagsKHR:8" => "VkGeometryInstanceFlagsKHR"
};

#[derive(Eq, Hash, PartialEq, Debug, serde::Deserialize)]
pub struct VkTypedef {
    #[serde(rename = "@category")]
    category: String,

    #[serde(rename = "type")]
    type_: String,

    #[serde(rename = "name")]
    name: String,
}

impl VkTypedef {
    pub fn fix(&mut self) {
        if self.category == "handle" {
            match self.type_.as_str() {
                "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => {
                    self.type_ = "VkNonDispatchableHandle".into()
                }
                "VK_DEFINE_HANDLE" => self.type_ = "VkDispatchableHandle".into(),
                &_ => unreachable!("handle type {}", self.type_),
            };
        }
    }
}

impl ToTokens for VkTypedef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", &self.name);
        let Ok(ffi_type) : Result<TokenStream, _> = C_TYPE_TO_RUST.get(&self.type_).unwrap_or(&"std::ffi::c_void").parse() else { unreachable!() };
        tokens.extend(quote! {
            type #name = #ffi_type;
        });
    }
}

#[derive(Debug, Default)]
pub struct VkXML {
    enums: HashSet<VkEnumeration>,
    structs: HashSet<VkStruct>,
    typedefs: HashSet<VkTypedef>,
}

impl Display for VkXML {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for enumeration in &self.enums {
            let tokens = vk_enum_to_rust(enumeration);
            writeln!(f, "{}", tokens).unwrap();
        }
        for s in &self.structs {
            let tokens = vk_struct_to_rust(s);
            writeln!(f, "{}", tokens).unwrap();
        }
        for b in &self.typedefs {
            let tokens = vk_base_type_to_rust(b);
            writeln!(f, "{}", tokens).unwrap();
        }
        Ok(())
    }
}

pub fn vk_enum_to_rust(enumeration: &VkEnumeration) -> TokenStream {
    let name = &enumeration.name;
    if !name.starts_with("Vk") {
        // TODO: Implement API constants.
        return TokenStream::new();
    }
    enumeration.into_token_stream()
}

pub fn vk_struct_to_rust(s: &VkStruct) -> TokenStream {
    s.into_token_stream()
}

pub fn vk_base_type_to_rust(b: &VkTypedef) -> TokenStream {
    b.into_token_stream()
}

#[derive(Error, Debug)]
pub enum ParseVkXMLError {
    #[error("XML read error: {0}")]
    XMLError(#[from] quick_xml::Error),
    #[error("UTF-8 error: {0}")]
    UTF8Error(#[from] Utf8Error),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] DeError),
}

pub fn read_vk_xml(str: &str) -> Result<VkXML, ParseVkXMLError> {
    let mut vk_xml: VkXML = Default::default();

    let mut reader = Reader::from_str(str);
    reader.trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"enums" => {
                    let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                    let str = std::str::from_utf8(&bytes)?;
                    let e: VkEnumeration = quick_xml::de::from_str(str)?;
                    vk_xml.enums.insert(e);
                }
                b"type" => {
                    let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                    let str = std::str::from_utf8(&bytes)?;
                    if let Ok(mut s) = quick_xml::de::from_str::<VkStruct>(str) {
                        if s.name == "VkPhysicalDeviceProperties" {
                            // HIRO
                            dbg!(&s);
                        }
                        if s.category == "struct" {
                            s.fix();
                            vk_xml.structs.insert(s);
                        }
                    }
                    if let Ok(mut b) = quick_xml::de::from_str::<VkTypedef>(str) {
                        if ["basetype", "bitmask", "handle"].contains(&b.category.as_str()) {
                            b.fix();
                            vk_xml.typedefs.insert(b);
                        }
                    }
                }
                b"command" => {
                    let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                    let str = std::str::from_utf8(&bytes)?;
                    // TODO: Parse functions
                }
                _ => (), // TODO: functions
            },
            _ => (),
        }
        buf.clear();
    }

    Ok(vk_xml)
}

fn read_to_end_into_buffer<R: BufRead>(
    reader: &mut Reader<R>,
    start_tag: &BytesStart,
) -> Result<Vec<u8>, ParseVkXMLError> {
    // NOTE: See also https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/
    let mut depth = 0;
    let mut output_buf: Vec<u8> = Vec::new(); // TODO: Zero-allocation XML read.
    let mut w = Writer::new(&mut output_buf);
    let tag_name = start_tag.name();
    w.write_event(Event::Start(start_tag.clone()))?;

    let mut temp_buf: Vec<u8> = Vec::new();
    loop {
        temp_buf.clear();
        let event = reader.read_event_into(&mut temp_buf)?;
        w.write_event(&event)?;

        match event {
            Event::Start(e) if e.name() == tag_name => depth += 1,
            Event::End(e) if e.name() == tag_name => {
                if depth == 0 {
                    return Ok(output_buf);
                }
                depth -= 1;
            }
            Event::Eof => {
                panic!()
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn works_codegen() {
        let vk_xml_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vk.xml"));
        let vk_xml_str = fs::read_to_string(vk_xml_path).unwrap();
        let vk_xml = read_vk_xml(&vk_xml_str).unwrap();

        for enumeration in &vk_xml.enums {
            if let Some(enumerators) = &enumeration.enumerators {
                for enumeration in enumerators {
                    assert!(enumeration.name.starts_with("VK_"), "{:?}", enumeration);
                    if let Some(ref alias) = enumeration.alias {
                        assert!(alias.starts_with("VK_"), "{:?}", enumeration);
                        assert!(enumeration.type_.is_none(), "{:?}", enumeration);
                    }
                    if let Some(ref type_) = enumeration.type_ {
                        assert!(
                            ["uint32_t", "float", "uint64_t"].contains(&type_.as_str()),
                            "{:?}",
                            enumeration
                        );
                    }
                }
            }
            let tokens = vk_enum_to_rust(enumeration);
        }

        for s in &vk_xml.structs {
            if let Some(members) = &s.members {
                for member in members {
                    if let Some(ref type_) = member.type_ {
                        assert!(
                            type_.starts_with("Vk")
                                || type_.starts_with("PFN_")
                                || type_.starts_with("Std")
                                || [
                                    "uint8_t",
                                    "uint16_t",
                                    "uint32_t",
                                    "uint64_t",
                                    "size_t",
                                    "int32_t",
                                    "int64_t",
                                    "float",
                                    "char",
                                    "void*",
                                    "uint32_t[2]",
                                    "uint32_t[3]",
                                    "float[2]",
                                    "float[3][4]",
                                    "uint32_t:24",
                                    "uint32_t:8",
                                ]
                                .contains(&type_.as_str()),
                            "{:?}",
                            member
                        );
                    }
                }
            }
            dbg!(&s);
            let tokens = vk_struct_to_rust(s);
            println!("tokens: {}", tokens);
        }
    }
}
