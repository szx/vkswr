use lazy_static::lazy_static;
use phf::phf_map;
use proc_macro2::{Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{DeError, Reader, Writer};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use regex::Replacer;
use serde::Deserialize;
use std::borrow::Cow;
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

impl VkEnumeration {
    fn fix(&mut self) {
        if let Some(ref mut enumerators) = self.enumerators {
            for enumerator in enumerators {
                enumerator.fix();
            }
        }
    }
}

impl ToTokens for VkEnumeration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(ref enumerators) = &self.enumerators {
            if !self.name.starts_with("Vk") {
                for enumerator in enumerators {
                    tokens.extend(enumerator.constant_to_token());
                }
                return;
            }

            let name = format_ident!("{}", &self.name);
            tokens.extend(quote! {
                #[repr(C)]
                enum #name {
                    #(#enumerators)*
                }
            });
            for enumerator in enumerators {
                if let Some(alias) = enumerator.alias_to_tokens(&self.name) {
                    tokens.extend(quote! {
                        impl #name {
                            pub const #alias;
                        }
                    });
                }
            }
        } else if self.type_ == Some("bitmask".into()) && self.enumerators == None {
            let name = format_ident!("{}", &self.name);
            tokens.extend(quote! {
                type #name = VkFlags;
            });
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
    fn fix(&mut self) {
        let Some(type_) = &self.type_ else { return };
        if let Some(type_) = C_TYPE_TO_FFI.get(type_) {
            self.type_ = Some((*type_).into())
        }
    }

    fn alias_to_tokens(&self, enumeration_name: &str) -> Option<TokenStream> {
        if let Some(ref alias) = self.alias {
            let enumeration_name = format_ident!("{}", enumeration_name);
            let name = format_ident!("{}", self.name);
            let alias = format_ident!("{}", alias);
            Some(quote!(#name: #enumeration_name = #enumeration_name::#alias).to_token_stream())
        } else {
            None
        }
    }

    fn constant_to_token(&self) -> Option<TokenStream> {
        let Some(type_) = &self.type_ else { return None };
        let Some(value) = &self.value else { return None };
        let name = format_ident!("{}", self.name);
        let type_ = format_ident!("{}", type_);
        let value = Literal::u32_suffixed(value.parse().ok()?);
        Some(quote!(const #name: #type_ = #value;).to_token_stream())
    }
}

impl ToTokens for VkEnumerator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(ref alias) = self.alias {
            // Skip, we have to use associated constants (alias_to_tokens).
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

    // Too hard to deserialize using Serde, filled in in fix().
    #[serde(skip)]
    members: Vec<VkStructMember>,
}

impl VkStruct {
    pub fn fix(&mut self, raw: &str) {
        lazy_static! {
            static ref re_extract_member: regex::Regex =
                regex::Regex::new(r"<member[^>]*>(.*?)</member[^>]*>").unwrap();
            static ref re_extract_name: regex::Regex =
                regex::Regex::new(r"<name[^>]*>(.*?)</name[^>]*>").unwrap();
            static ref re_remove_comments: regex::Regex =
                regex::Regex::new(r"<comment[^>]*>(.*?)</comment[^>]*>").unwrap();
            static ref re_remove_tags: regex::Regex = regex::Regex::new(r"</?[^>]*>").unwrap();
            static ref re_replace_spaces: regex::Regex = regex::Regex::new(r"\s\s+").unwrap();
        }

        for cap in re_extract_member.captures_iter(raw) {
            let str = &cap[0];
            if str.contains("vulkansc") {
                continue;
            }

            let mut name = re_extract_name
                .captures(str)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();

            let type_ = re_remove_comments.replace_all(str, " ");
            let type_ = re_extract_name.replace_all(&type_, " ");
            let type_ = re_remove_tags.replace_all(&type_, " ");
            let type_ = re_replace_spaces.replace_all(&type_, " ");
            let type_ = type_.trim().to_string();

            if name == "type" {
                name = "type_".into();
            }

            let mut member = VkStructMember { type_, name };
            self.members.push(member);
        }

        let is_union = self.category == "union";
        let mut iter = self.members.iter_mut();
        while let Some(member) = iter.next() {
            if member.is_bitfield_24() {
                let next = iter.next();
                member.fix(next, &is_union);
            } else {
                member.fix(None, &is_union);
            }
        }
        self.members.retain(|x| !x.is_bitfield_8());
    }
}

impl ToTokens for VkStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", &self.name);
        let members = &self.members;
        if members.is_empty() {
            return;
        }
        if self.category == "struct" {
            tokens.extend(quote! {
                #[repr(C)]
                struct #name {
                    #(#members)*
                }
            });
        } else if self.category == "union" {
            tokens.extend(quote! {
                #[repr(C)]
                union #name {
                    #(#members)*
                }
            });
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct VkStructMember {
    type_: String,
    name: String,
}

impl VkStructMember {
    fn is_bitfield_24(&self) -> bool {
        self.type_.contains(" :24")
    }
    fn is_bitfield_8(&self) -> bool {
        self.type_.contains(" :8")
    }

    fn fix(&mut self, next: Option<&mut VkStructMember>, is_union: &bool) {
        lazy_static! {
            static ref re_replace_struct: regex::Regex = regex::Regex::new(r"struct\s").unwrap();
            static ref re_const_ptr: regex::Regex = regex::Regex::new(r"const\s(.*?)\s\*").unwrap();
            static ref re_mut_ptr: regex::Regex = regex::Regex::new(r"(.*?)\s\*").unwrap();
            static ref re_array: regex::Regex = regex::Regex::new(r"(.*?)\s\[(.*?)\]").unwrap();
            static ref re_bitfield_24: regex::Regex = regex::Regex::new(r"(.*?)\s\:24").unwrap();
            static ref re_bitfield_8: regex::Regex = regex::Regex::new(r"(.*?)\s\:8").unwrap();
        }

        let mut name: String = self.name.clone();
        let mut type_: String = self.type_.clone();
        type_ = re_replace_struct.replace_all(&type_, "").into();

        let is_const_ptr = if let Some(cap) = re_const_ptr.captures(&type_) {
            type_ = cap.get(1).unwrap().as_str().into();
            true
        } else {
            false
        };
        let is_mut_ptr = if let Some(cap) = re_mut_ptr.captures(&type_) {
            type_ = cap.get(1).unwrap().as_str().into();
            true
        } else {
            false
        };
        let array_index = if let Some(cap) = re_array.captures(&type_) {
            let i = cap.get(2).unwrap().as_str().to_string();
            type_ = cap.get(1).unwrap().as_str().into();
            Some(i)
        } else {
            None
        };
        let is_bitfield_24 = if let Some(cap) = re_bitfield_24.captures(&type_) {
            type_ = cap.get(1).unwrap().as_str().into();
            true
        } else {
            false
        };
        let ffi = C_TYPE_TO_FFI.get(&type_);

        if let Some(ffi) = ffi {
            type_.replace_range(.., ffi);
        }
        if is_bitfield_24 {
            if let Some(next) = next {
                name = format!("{}_{}", name, next.name);
            }
        }
        if let Some(array_index) = array_index {
            type_ = format!("[{}; {} as usize]", type_, array_index);
        }
        if is_const_ptr {
            type_ = format!("*const {}", type_);
        } else if is_mut_ptr {
            type_ = format!("*mut {}", type_);
        }
        if *is_union {
            type_ = format!("std::mem::ManuallyDrop<{}>", type_);
        }

        self.name = name;
        self.type_ = type_;
    }
}

static C_TYPE_TO_FFI: phf::Map<&'static str, &'static str> = phf_map! {
    "uint8_t" => "u8",
    "uint16_t" => "u16",
    "uint32_t" => "u32",
    "uint64_t" => "u64",
    "size_t" => "isize",
    "int32_t" => "i32",
    "int64_t" => "i64",
    "float" => "f32",
    "char" => "std::ffi::c_char",
    "void" => "std::ffi::c_void",
};

impl ToTokens for VkStructMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(format_ident!("{}", self.name));
        tokens.append(Punct::new(':', Spacing::Alone));
        tokens.extend(self.type_.parse::<TokenStream>());
        tokens.append(Punct::new(',', Spacing::Alone));
    }
}

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
        let Ok(ffi_type) : Result<TokenStream, _> = C_TYPE_TO_FFI.get(&self.type_).unwrap_or(&"std::ffi::c_void").parse() else { unreachable!() };
        tokens.extend(quote! {
            type #name = #ffi_type;
        });
    }
}

#[derive(Eq, Hash, PartialEq, Debug, serde::Deserialize)]
pub struct VkTypeDefinition {
    #[serde(rename = "@category")]
    category: Option<String>,

    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@alias")]
    alias: String,
}

impl ToTokens for VkTypeDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(category) = &self.category {
            let name = format_ident!("{}", self.name);
            let alias = format_ident!("{}", self.alias);
            tokens.extend(quote! {
                type #name = #alias;
            });
        }
    }
}

#[derive(Debug, Default)]
pub struct VkXML {
    enums: HashSet<VkEnumeration>,
    structs: HashSet<VkStruct>,
    typedefs: HashSet<VkTypedef>,
    type_definitions: HashSet<VkTypeDefinition>,
}

impl Display for VkXML {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in &self.enums {
            let tokens = x.into_token_stream();
            writeln!(f, "{}", tokens).unwrap();
        }
        for x in &self.structs {
            let tokens = x.into_token_stream();
            writeln!(f, "{}", tokens).unwrap();
        }
        for x in &self.typedefs {
            let tokens = x.into_token_stream();
            writeln!(f, "{}", tokens).unwrap();
        }
        for x in &self.type_definitions {
            let tokens = x.into_token_stream();
            writeln!(f, "{}", tokens).unwrap();
        }
        Ok(())
    }
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
    reader.expand_empty_elements(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                //dbg!(&e);
                match e.name().as_ref() {
                    b"enums" => {
                        let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                        let str = std::str::from_utf8(&bytes)?;
                        if let Ok(mut e) = quick_xml::de::from_str::<VkEnumeration>(str) {
                            e.fix();
                            vk_xml.enums.insert(e);
                        }
                    }
                    b"type" => {
                        let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                        let str = std::str::from_utf8(&bytes)?;
                        if let Ok(mut s) = quick_xml::de::from_str::<VkStruct>(str) {
                            if s.category == "struct" || s.category == "union" {
                                s.fix(str);
                                vk_xml.structs.insert(s);
                            }
                        }
                        if let Ok(mut b) = quick_xml::de::from_str::<VkTypedef>(str) {
                            if ["basetype", "bitmask", "handle"].contains(&b.category.as_str()) {
                                b.fix();
                                vk_xml.typedefs.insert(b);
                            }
                        }
                        if let Ok(mut x) = quick_xml::de::from_str::<VkTypeDefinition>(str) {
                            vk_xml.type_definitions.insert(x);
                        }
                    }
                    b"command" => {
                        let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                        let str = std::str::from_utf8(&bytes)?;
                        // TODO: Parse functions
                    }
                    _ => (), // TODO: functions
                }
            }
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
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::{fs, io};

    #[test]
    fn works_ffi_type() {
        fn test_case(
            input: (&str, &str),
            next_input: Option<(&str, &str)>,
            expected: (&str, &str),
        ) {
            let mut s = VkStructMember {
                type_: input.0.into(),
                name: input.1.into(),
            };
            if let Some(next_input) = next_input {
                let mut next = VkStructMember {
                    type_: next_input.0.into(),
                    name: next_input.1.into(),
                };
                s.fix(Some(&mut next), &false);
            } else {
                s.fix(None, &false);
            }
            assert_eq!((s.type_.as_str(), s.name.as_str()), expected, "{:?}", s);
        }
        test_case(("uint32_t", "foo"), None, ("u32", "foo"));
        test_case(("int32_t *", "foo"), None, ("*mut i32", "foo"));
        test_case(
            ("const VkBaseOutStructure *", "foo"),
            None,
            ("*const VkBaseOutStructure", "foo"),
        );
        test_case(("LPCWSTR", "foo"), None, ("LPCWSTR", "foo"));
        test_case(
            ("uint8_t [VK_UUID_SIZE]", "foo"),
            None,
            ("[u8; VK_UUID_SIZE]", "foo"),
        );
        test_case(
            ("uint32_t :24", "foo"),
            Some(("VkGeometryInstanceFlagsKHR :8", "bar")),
            ("u32", "foo_bar"),
        );
        // HIRO function pointer
        // HIRO unknown
    }

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
                            ["u32", "f32", "u64"].contains(&type_.as_str()),
                            "{:?}",
                            enumeration
                        );
                    }
                }
            }
            let tokens = enumeration.into_token_stream();
        }

        for s in &vk_xml.structs {
            for member in &s.members {
                let ffi_type = &member.type_;
                assert_eq!(ffi_type, ffi_type.trim());
                assert_ne!(ffi_type.chars().last().unwrap(), '*');
            }
            let tokens = s.into_token_stream();
        }

        writeln!(io::sink(), "vk_xml: {}", vk_xml).unwrap();
    }
}
