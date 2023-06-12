use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::fs;
use std::io::BufRead;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
struct VkEnumeration {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@type")]
    type_: Option<String>,

    #[serde(rename = "enum")]
    //#[serde(rename = "$value")]
    enum1: Option<Vec<VkEnumerator>>,
}

#[derive(Debug, serde::Deserialize)]
struct VkEnumerator {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@alias")]
    alias: Option<String>,

    #[serde(rename = "@type")]
    type_: Option<String>,

    #[serde(rename = "@value")]
    value: Option<String>,
}

fn read_vk_xml(str: &str) -> Vec<VkEnumeration> {
    let mut enums: Vec<VkEnumeration> = vec![];

    let mut reader = Reader::from_str(str);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut junk_buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"enums" => {
                    let bytes = read_to_end_into_buffer(&mut reader, &e, &mut junk_buf).unwrap();
                    let str = std::str::from_utf8(&bytes).unwrap();
                    let e: VkEnumeration = quick_xml::de::from_str(str).unwrap();
                    enums.push(e);
                }
                _ => (), // TODO: Parse structs, functions
            },
            _ => (),
        }
        buf.clear();
    }

    enums
}

fn read_to_end_into_buffer<R: BufRead>(
    reader: &mut Reader<R>,
    start_tag: &BytesStart,
    junk_buf: &mut Vec<u8>,
) -> Result<Vec<u8>, quick_xml::Error> {
    let mut depth = 0;
    let mut output_buf: Vec<u8> = Vec::new();
    let mut w = Writer::new(&mut output_buf);
    let tag_name = start_tag.name();
    w.write_event(Event::Start(start_tag.clone()))?;
    loop {
        junk_buf.clear();
        let event = reader.read_event_into(junk_buf)?;
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
    #[test]
    fn works_codegen() {
        let path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vk.xml"));
        let str = fs::read_to_string(path).unwrap();
        let enums = read_vk_xml(&str);
        for e in enums {
            dbg!(&e);
            if let Some(enumerators) = e.enum1 {
                for enumeration in enumerators {
                    assert!(enumeration.name.starts_with("VK_"), "{:?}", enumeration);
                    if let Some(ref alias) = enumeration.alias {
                        assert!(alias.starts_with("VK_"), "{:?}", enumeration);
                        assert!(enumeration.type_.is_none(), "{:?}", enumeration);
                    }
                    if let Some(ref type_) = enumeration.type_ {
                        assert!(["uint32_t", "float", "uint64_t"].contains(&type_.as_str()), "{:?}", enumeration);
                    }
                }
            }
        }
        // TODO: Create functions using quote crate.
        // TODO: Create stubs.rs.
    }
}
