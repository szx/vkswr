use proc_macro2::{Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{DeError, Reader, Writer};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::fs;
use std::io::BufRead;
use std::path::PathBuf;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, serde::Deserialize)]
struct VkEnumeration {
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
            let t = quote! {
                #[repr(C)]
                enum #name {
                    #(#enumerators),*
                }
            };
            tokens.extend(t);
        }
    }
}

#[derive(Debug, serde::Deserialize)]
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

impl ToTokens for VkEnumerator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(format_ident!("{}", self.name));

        if let Some(ref alias) = self.alias {
            tokens.append(Punct::new('=', Spacing::Alone));
            tokens.append(format_ident!("{}", alias));
        } else if let Some(ref value) = self.value {
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
        }
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

fn read_vk_xml(str: &str) -> Result<Vec<VkEnumeration>, ParseVkXMLError> {
    let mut enums: Vec<VkEnumeration> = vec![];

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
                    enums.push(e);
                }
                _ => (), // TODO: Parse structs, functions
            },
            _ => (),
        }
        buf.clear();
    }

    Ok(enums)
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

fn vk_enum_to_rust(enumeration: &VkEnumeration) -> TokenStream {
    let name = &enumeration.name;
    if !name.starts_with("Vk") {
        // TODO: Implement API constants.
        return TokenStream::new();
    }
    enumeration.into_token_stream()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn works_codegen() {
        let vk_xml_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vk.xml"));
        let enums = read_vk_xml(&fs::read_to_string(vk_xml_path).unwrap()).unwrap();

        for enumeration in &enums {
            //dbg!(&enumeration);
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
            // TODO: Write to stubs files.
            println!("tokens: {}", &tokens.to_string());
        }
    }
}

// TODO: Create stubs.rs.
