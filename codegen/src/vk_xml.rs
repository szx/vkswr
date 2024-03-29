use lazy_static::lazy_static;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use serde::Deserialize;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::BufRead;
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct VkXml {
    pub enums: Vec<VkEnums>,
    pub structs: HashSet<VkStruct>,
    pub typedefs: HashSet<VkTypedef>,
    pub type_attributes: HashSet<VkTypeAttributes>,
    pub type_externs: HashSet<VkTypeExtern>,
    pub commands: HashSet<VkCommand>,
    pub extensions: HashSet<VkExtension>,
}

impl VkXml {
    /// # Errors
    ///
    /// Will return `Err` if opening, reading, or parsing of file with path `vk_xml_path` fails.
    pub fn from(vk_xml_path: impl AsRef<Path>) -> Result<Self, ParseVkXmlError> {
        let vk_xml_path = vk_xml_path.as_ref();
        let vk_xml_str = std::fs::read_to_string(vk_xml_path)?;
        let vk_xml = read_vk_xml(&vk_xml_str)?;
        Ok(vk_xml)
    }
}

#[derive(Error, Debug)]
pub enum ParseVkXmlError {
    #[error("IO read error: {0}")]
    IO(#[from] std::io::Error),
    #[error("XML read error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("UTF-8 error: {0}")]
    UTF8(#[from] std::str::Utf8Error),
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] quick_xml::DeError),
}

fn read_vk_xml(str: &str) -> Result<VkXml, ParseVkXmlError> {
    let mut vk_xml: VkXml = VkXml::default();

    let mut reader = Reader::from_str(str);
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"enums" => {
                    let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                    let str = std::str::from_utf8(&bytes)?;
                    if let Ok(e) = quick_xml::de::from_str::<VkXmlEnumsSerde>(str) {
                        vk_xml.enums.push(e.fix());
                    }
                }
                b"type" => {
                    let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                    let str = std::str::from_utf8(&bytes)?;
                    if let Ok(mut x) = quick_xml::de::from_str::<VkStructSerde>(str) {
                        if let Some(x) = x.fix(str) {
                            vk_xml.structs.insert(x);
                            continue;
                        }
                    }
                    if let Ok(x) = quick_xml::de::from_str::<VkTypedefSerde>(str) {
                        if let Some(x) = x.fix() {
                            vk_xml.typedefs.insert(x);
                        }
                        continue;
                    }
                    if let Ok(x) = quick_xml::de::from_str::<VkTypeAttributesSerde>(str) {
                        vk_xml.type_attributes.insert(x.fix());
                        continue;
                    }
                    if let Ok(x) = quick_xml::de::from_str::<VkTypeExternSerde>(str) {
                        vk_xml.type_externs.insert(x.fix());
                        continue;
                    }
                    if let Ok(x) = quick_xml::de::from_str::<VkTypeExternSerde2>(str) {
                        if let Some(x) = x.fix(str) {
                            vk_xml.type_externs.insert(x);
                        }
                        continue;
                    }
                    if str.contains(r#"category="define""#) || str.contains(r#"name=""#) {
                        continue;
                    }
                    unreachable!("{:?}", str);
                }
                b"command" => {
                    let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                    let str = std::str::from_utf8(&bytes)?;
                    if let Ok(x) = quick_xml::de::from_str::<VkCommandSerde>(str) {
                        if let Some(x) = x.fix(str) {
                            vk_xml.commands.insert(x);
                        }
                        continue;
                    }
                }
                b"extension" | b"feature" => {
                    let bytes = read_to_end_into_buffer(&mut reader, &e)?;
                    let str = std::str::from_utf8(&bytes)?;
                    if let Ok(x) = quick_xml::de::from_str::<VkExtensionSerde>(str) {
                        if let Some(x) = x.fix() {
                            vk_xml.extensions.insert(x);
                        }
                        continue;
                    }
                }
                _ => (),
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
) -> Result<Vec<u8>, ParseVkXmlError> {
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

// ---

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
struct VkXmlEnumsSerde {
    #[serde(rename = "@name")]
    name: Arc<str>,

    #[serde(rename = "@type")]
    type_: Option<Arc<str>>,

    #[serde(rename = "enum")]
    //#[serde(rename = "$value")]
    enumerators: Option<Vec<VkXmlEnumsMemberSerde>>,
}

impl VkXmlEnumsSerde {
    pub fn fix(&self) -> VkEnums {
        let Some(type_) = &self.type_ else {
            return VkEnums::ApiConstants {
                members: self
                    .enumerators
                    .as_ref()
                    .expect("at least one enumerator")
                    .iter()
                    .map(|x| x.fix(self.name.clone()))
                    .collect(),
            };
        };
        let type_ = type_.as_ref();
        if type_ == "enum" || type_ == "bitmask" {
            VkEnums::Enum {
                name: self.name.clone(),
                members: self
                    .enumerators
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|x| x.fix(self.name.clone()))
                    .collect(),
            }
        } else {
            unreachable!("{:?}", self)
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkEnums {
    ApiConstants {
        members: Vec<VkEnumsMember>,
    },
    Enum {
        name: Arc<str>,
        members: Vec<VkEnumsMember>,
    },
}

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
struct VkXmlEnumsMemberSerde {
    #[serde(rename = "@name")]
    name: Arc<str>,

    #[serde(rename = "@value")]
    value: Option<Arc<str>>,

    #[serde(rename = "@bitpos")]
    bitpos: Option<Arc<str>>,

    #[serde(rename = "@type")]
    type_: Option<Arc<str>>,

    #[serde(rename = "@alias")]
    alias: Option<Arc<str>>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkEnumsMember {
    MemberValue {
        name: Arc<str>,
        value: Option<String>,
        enum_name: Arc<str>,
    },
    MemberBitpos {
        name: Arc<str>,
        bitpos: Arc<str>,
        enum_name: Arc<str>,
    },
    Alias {
        name: Arc<str>,
        alias: Arc<str>,
        enum_name: Arc<str>,
    },
    ApiConstantAlias {
        name: Arc<str>,
        alias: Arc<str>,
    },
    ApiConstantMember {
        name: Arc<str>,
        type_: VkFFIType,
        value: String,
    },
}

impl VkXmlEnumsMemberSerde {
    pub fn fix(&self, enum_name: Arc<str>) -> VkEnumsMember {
        if enum_name.as_ref() == "API Constants" {
            self.alias.as_ref().map_or_else(
                || VkEnumsMember::ApiConstantMember {
                    name: self.name.clone(),
                    type_: VkFFIType::new(self.type_.clone().expect("type").as_ref()),
                    value: VkFFIType::parse_value(&self.value.as_ref().expect("type").clone()),
                },
                |alias| VkEnumsMember::ApiConstantAlias {
                    name: self.name.clone(),
                    alias: alias.clone(),
                },
            )
        } else if let Some(alias) = &self.alias {
            VkEnumsMember::Alias {
                name: self.name.clone(),
                alias: alias.clone(),
                enum_name,
            }
        } else if let Some(bitpos) = &self.bitpos {
            VkEnumsMember::MemberBitpos {
                name: self.name.clone(),
                bitpos: bitpos.clone(),
                enum_name,
            }
        } else {
            VkEnumsMember::MemberValue {
                name: self.name.clone(),
                value: self.value.as_deref().map(|x| x.to_string()),
                enum_name,
            }
        }
    }
}

// ---

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
struct VkStructSerde {
    #[serde(rename = "@category")]
    category: Arc<str>,

    #[serde(rename = "@name")]
    name: Arc<str>,

    #[serde(rename = "@alias")]
    alias: Option<Arc<str>>,
    // Too hard to deserialize members using Serde, filled in in fix().
}

impl VkStructSerde {
    fn fix(&mut self, str: &str) -> Option<VkStruct> {
        let members = Self::parse_members(str, self.category.as_ref() == "union");
        let name = self.name.clone();
        match self.category.as_ref() {
            "include" | "define" => None,
            "enum" if self.alias.is_none() => None,
            "bitmask" | "handle" | "enum" => {
                let alias = self.alias.as_ref().expect("alias").clone();
                Some(VkStruct::Alias { name, alias })
            }
            "struct" if members.is_empty() => None,
            "struct" => Some(VkStruct::Struct { name, members }),
            "union" => Some(VkStruct::Union { name, members }),
            _ => unreachable!("{:?} {}", self, str),
        }
    }

    fn parse_members(str: &str, is_union: bool) -> Vec<VkStructMember> {
        lazy_static! {
            static ref RE_MEMBER: regex::Regex =
                regex::Regex::new(r"<member[^>]*>(.*?)</member[^>]*>").expect("regex");
            static ref RE_NAME: regex::Regex =
                regex::Regex::new(r"<name[^>]*>(.*?)</name[^>]*>").expect("regex");
            static ref RE_COMMENT: regex::Regex =
                regex::Regex::new(r"<comment[^>]*>(.*?)</comment[^>]*>").expect("regex");
            static ref RE_ALL_TAGS: regex::Regex = regex::Regex::new(r"</?[^>]*>").expect("regex");
            static ref RE_ALL_SPACES: regex::Regex = regex::Regex::new(r"\s\s+").expect("regex");
        }
        let mut members = vec![];
        for cap in RE_MEMBER.captures_iter(str) {
            let str = &cap[0];
            if str.contains("vulkansc") {
                continue;
            }
            let mut name = RE_NAME
                .captures(str)
                .expect("captures")
                .get(1)
                .expect("capture")
                .as_str()
                .to_string();
            let type_ = RE_COMMENT.replace_all(str, " ");
            let type_ = RE_NAME.replace_all(&type_, " ");
            let type_ = RE_ALL_TAGS.replace_all(&type_, " ");
            let type_ = RE_ALL_SPACES.replace_all(&type_, " ");
            let type_ = type_.trim().to_string();
            if name == "type" {
                name = "type_".into();
            }
            let member = VkStructMemberSerde { type_, name };
            members.push(member);
        }

        let mut iter = members.iter_mut();
        let mut members: Vec<VkStructMember> = vec![];
        while let Some(member) = iter.next() {
            if member.is_bitfield_24() {
                let next = iter.next();
                members.push(member.fix(next, is_union));
            } else {
                members.push(member.fix(None, is_union));
            }
        }
        members
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct VkStructMemberSerde {
    type_: String,
    name: String,
}

impl VkStructMemberSerde {
    fn is_bitfield_24(&self) -> bool {
        self.type_.contains(" :24")
    }

    fn fix(&mut self, next: Option<&mut Self>, in_union: bool) -> VkStructMember {
        lazy_static! {
            static ref RE_BITFIELD_24: regex::Regex =
                regex::Regex::new(r"(.*?)\s\:24").expect("regex");
        }

        if let Some(cap) = RE_BITFIELD_24.captures(&self.type_) {
            self.type_ = cap.get(1).expect("capture").as_str().into();
            if let Some(next) = next {
                self.name = format!("{}_{}", self.name, next.name);
            }
        }

        let name = self.name.clone().into();
        let type_ = VkFFIType::new(&self.type_);

        VkStructMember::Member {
            name,
            type_,
            in_union,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkStruct {
    Alias {
        name: Arc<str>,
        alias: Arc<str>,
    },
    Struct {
        name: Arc<str>,
        members: Vec<VkStructMember>,
    },
    Union {
        name: Arc<str>,
        members: Vec<VkStructMember>,
    },
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkStructMember {
    Member {
        name: Arc<str>,
        type_: VkFFIType,
        in_union: bool,
    },
}

// ---

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
pub struct VkTypeAttributesSerde {
    #[serde(rename = "@category")]
    category: Arc<str>,

    #[serde(rename = "@name")]
    name: Arc<str>,

    #[serde(rename = "@alias")]
    alias: Arc<str>,
}

impl VkTypeAttributesSerde {
    #[must_use]
    pub fn fix(&self) -> VkTypeAttributes {
        VkTypeAttributes::Alias {
            name: self.name.clone(),
            alias: self.alias.clone(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkTypeAttributes {
    Alias { name: Arc<str>, alias: Arc<str> },
}

// ---

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
pub struct VkTypeExternSerde {
    #[serde(rename = "@requires")]
    requires: Arc<str>,

    #[serde(rename = "@name")]
    name: Arc<str>,
}

impl VkTypeExternSerde {
    #[must_use]
    pub fn fix(&self) -> VkTypeExtern {
        VkTypeExtern::Extern {
            name: self.name.clone(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
pub struct VkTypeExternSerde2 {
    #[serde(rename = "@category")]
    category: Arc<str>,

    #[serde(rename = "name")]
    name: Arc<str>,

    // Too hard to deserialize using Serde, filled in in fix().
    #[serde(skip)]
    members: Vec<VkStructMemberSerde>,
}

impl VkTypeExternSerde2 {
    #[must_use]
    pub fn fix(&self, str: &str) -> Option<VkTypeExtern> {
        match self.category.as_ref() {
            "basetype" => Some(VkTypeExtern::Extern {
                name: self.name.clone(),
            }),
            "define" => None,
            "funcpointer" => VkFuncDeclMember::parse_members(str),
            &_ => {
                unreachable!("{:?}", self)
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkTypeExtern {
    Extern {
        name: Arc<str>,
    },
    FuncPointer {
        type_: Option<VkFFIType>,
        name: Arc<str>,
        members: Vec<VkFuncDeclMember>,
    },
}

// ---

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
pub struct VkCommandSerde {
    #[serde(rename = "@queues")]
    queues: Option<Arc<str>>,

    #[serde(rename = "@renderpass")]
    renderpass: Option<Arc<str>>,

    #[serde(rename = "@cmdbufferlevel")]
    cmdbufferlevel: Option<Arc<str>>,

    #[serde(rename = "@tasks")]
    tasks: Option<Arc<str>>,

    #[serde(rename = "@successcodes")]
    successcodes: Option<Arc<str>>,

    #[serde(rename = "@errorcodes")]
    errorcodes: Option<Arc<str>>,
}

impl VkCommandSerde {
    #[must_use]
    pub fn fix(&self, str: &str) -> Option<VkCommand> {
        VkCommand::parse(str)
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkCommand {
    Command {
        type_: Option<VkFFIType>,
        name: Arc<str>,
        members: Vec<VkFuncDeclMember>,
    },
}

impl VkCommand {
    fn parse(str: &str) -> Option<Self> {
        lazy_static! {
            static ref RE_PROTO: regex::Regex =
                regex::Regex::new(r"<proto[^>]*>(.*?)<\/proto[^>]*>").expect("regex");
            static ref RE_NAME: regex::Regex =
                regex::Regex::new(r"<name[^>]*>(.*?)<\/name[^>]*>").expect("regex");
            static ref RE_PARAM: regex::Regex =
                regex::Regex::new(r"<param[^>]*>(.*?)<\/param[^>]*>").expect("regex");
            static ref RE_IMPLICITEXTERNSYNCPARAMS: regex::Regex = regex::Regex::new(
                r"(?s)<implicitexternsyncparams[^>]*>(.*?)<\/implicitexternsyncparams[^>]*>"
            )
            .expect("regex");
            static ref RE_ALL_TAGS: regex::Regex = regex::Regex::new(r"</?[^>]*>").expect("regex");
            static ref RE_ALL_SPACES: regex::Regex = regex::Regex::new(r"\s\s+").expect("regex");
            static ref RE_MEMBER: regex::Regex =
                regex::Regex::new(r"\s?(.*?)\s(\w*?)[,)]").expect("regex");
        }

        let Some(cap) = RE_PROTO.captures(str.as_ref()) else {
            return None;
        };
        let proto = cap.get(1).expect("capture").as_str();

        let Some(cap) = RE_NAME.captures(proto) else {
            return None;
        };
        let name = cap.get(1).expect("capture").as_str().into();

        let proto = RE_NAME.replace_all(proto, " ");
        let proto = RE_ALL_TAGS.replace_all(proto.as_ref(), " ");
        let proto = RE_ALL_SPACES.replace_all(proto.as_ref(), " ");
        let type_ = proto.trim();
        let type_ = if type_ == "void" {
            None
        } else {
            Some(VkFFIType::new(type_))
        };

        let mut members: Vec<VkFuncDeclMember> = vec![];
        let str = RE_IMPLICITEXTERNSYNCPARAMS.replace_all(str.as_ref(), " ");
        for cap in RE_PARAM.captures_iter(str.as_ref()) {
            if cap[0].contains("vulkansc") {
                continue;
            }

            let Some(cap_name) = RE_NAME.captures(&cap[0]) else {
                unreachable!("{:?}", &cap[0])
            };
            let mut name = cap_name.get(1).expect("capture").as_str().into();
            if name == "type" {
                name = "type_".into();
            }

            let type_ = RE_NAME.replace_all(&cap[0], " ");
            let type_ = RE_ALL_TAGS.replace_all(type_.as_ref(), " ");
            let type_ = RE_ALL_SPACES.replace_all(type_.as_ref(), " ");
            let type_ = VkFFIType::new(type_.as_ref());

            members.push(VkFuncDeclMember::Member { name, type_ });
        }

        Some(Self::Command {
            type_,
            name,
            members,
        })
    }
}

// ---

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkFuncDeclMember {
    Member { name: String, type_: VkFFIType },
}

impl VkFuncDeclMember {
    fn parse_members(str: &str) -> Option<VkTypeExtern> {
        lazy_static! {
            static ref RE_ALL_TAGS: regex::Regex = regex::Regex::new(r"</?[^>]*>").expect("regex");
            static ref RE_ALL_SPACES: regex::Regex = regex::Regex::new(r"\s\s+").expect("regex");
            static ref RE_TYPE_NAME_MEMBERS: regex::Regex = regex::Regex::new(
                r"typedef\s(.*?)\s\(VKAPI_PTR\s\*\s(.*?)\s\)\((?:\s|void)(.*?)\);"
            )
            .expect("regex");
            static ref RE_MEMBER: regex::Regex =
                regex::Regex::new(r"\s?(.*?)\s(\w*?)[,)]").expect("regex");
        }

        let str = RE_ALL_TAGS.replace_all(str, " ");
        let str = RE_ALL_SPACES.replace_all(&str, " ");
        let Some(cap) = RE_TYPE_NAME_MEMBERS.captures(str.as_ref()) else {
            return None;
        };

        let type_ = cap.get(1).expect("capture").as_str();
        let type_ = if type_ == "void" {
            None
        } else {
            Some(VkFFIType::new(type_))
        };

        let name = cap.get(2).expect("capture").as_str().into();

        let raw_members = cap.get(3).expect("capture").as_str();
        let mut members: Vec<Self> = vec![];
        for cap in RE_MEMBER.captures_iter(raw_members) {
            let type_ = VkFFIType::new(cap[1].into());
            let name = cap[2].into();
            members.push(Self::Member { name, type_ });
        }

        Some(VkTypeExtern::FuncPointer {
            type_,
            name,
            members,
        })
    }
}

// ---

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct VkFFIType(pub String);

impl VkFFIType {
    fn new(str: &str) -> Self {
        lazy_static! {
            static ref RE_STRUCT: regex::Regex = regex::Regex::new(r"struct\s").expect("regex");
            static ref RE_CONST_PTR: regex::Regex =
                regex::Regex::new(r"const\s(.*?)\s\*").expect("regex");
            static ref RE_MUT_PTR: regex::Regex = regex::Regex::new(r"(.*?)\s?\*").expect("regex");
            static ref RE_FIXED_ARRAY: regex::Regex =
                regex::Regex::new(r"(.*?)\s\[(.*?)\]").expect("regex");
            static ref RE_CONST: regex::Regex = regex::Regex::new(r"const\s").expect("regex");
        }

        let mut type_: String = str.to_string();
        type_ = RE_STRUCT.replace_all(&type_, "").into();
        let mut is_const_ptr = false;
        if let Some(cap) = RE_CONST_PTR.captures(&type_) {
            is_const_ptr = true;
            type_ = cap.get(1).expect("capture").as_str().into();
        }
        let mut is_mut_ptr = false;
        if let Some(cap) = RE_MUT_PTR.captures(&type_) {
            is_mut_ptr = true;
            type_ = cap.get(1).expect("capture").as_str().into();
        }
        let mut array_index = None;
        if let Some(cap) = RE_FIXED_ARRAY.captures(&type_) {
            let i = cap.get(2).expect("capture").as_str().to_string();
            array_index = Some(i);
            type_ = cap.get(1).expect("capture").as_str().into();
        }
        type_ = RE_CONST.replace_all(&type_, "").trim().into();
        let ffi = c_type_to_ffi(&type_);

        if let Some(ffi) = ffi {
            type_.replace_range(.., ffi);
        }
        if let Some(array_index) = array_index {
            type_ = format!("[{type_}; {array_index} as usize]");
        }
        if is_const_ptr || is_mut_ptr {
            type_ = format!("Option<NonNull<{type_}>>");
        }

        Self(type_)
    }

    fn parse_value(value: &Arc<str>) -> String {
        parse_int::parse::<usize>(value.as_ref()).map_or_else(
            |_| {
                if value.ends_with('F') {
                    format!("{}_f32", value.split('F').next().expect("float"))
                } else if value.as_ref() == "(~0U)" {
                    "!0_u32".into()
                } else if value.as_ref() == "(~0ULL)" {
                    "!0_u64".into()
                } else if value.as_ref() == "(~1U)" {
                    "!1_u32".into()
                } else if value.as_ref() == "(~2U)" {
                    "!2_u32".into()
                } else {
                    unreachable!("{:?}", value)
                }
            },
            |value| format!("{}", value),
        )
    }
}

fn c_type_to_ffi(from: &str) -> Option<&'static str> {
    match from {
        "uint8_t" => Some("u8"),
        "uint16_t" => Some("u16"),
        "uint32_t" => Some("u32"),
        "uint64_t" => Some("u64"),
        "size_t" => Some("isize"),
        "int32_t" => Some("i32"),
        "int64_t" => Some("i64"),
        "float" => Some("f32"),
        "char" => Some("std::ffi::c_char"),
        "void" => Some("std::ffi::c_void"),
        _ => None,
    }
}

// ---

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
pub struct VkTypedefSerde {
    #[serde(rename = "@category")]
    category: Arc<str>,

    #[serde(rename = "type")]
    type_: Arc<str>,

    #[serde(rename = "name")]
    name: Arc<str>,
}

impl VkTypedefSerde {
    #[must_use]
    pub fn fix(&self) -> Option<VkTypedef> {
        match self.category.as_ref() {
            "define" => None,
            "basetype" | "bitmask" => Some(VkTypedef::Alias {
                name: self.name.clone(),
                type_: VkFFIType::new(self.type_.as_ref()),
            }),
            "handle" => {
                let type_ = match self.type_.as_ref() {
                    "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => "VkNonDispatchableHandle",
                    "VK_DEFINE_HANDLE" => "VkDispatchableHandle",
                    &_ => {
                        unreachable!("{:?}", self)
                    }
                };
                Some(VkTypedef::Alias {
                    name: self.name.clone(),
                    type_: VkFFIType::new(type_),
                })
            }
            &_ => {
                unreachable!("{:?}", self)
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkTypedef {
    Alias { name: Arc<str>, type_: VkFFIType },
}

// ---

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
pub struct VkExtensionSerde {
    #[serde(rename = "@name")]
    name: Arc<str>,

    #[serde(rename = "@number")]
    number: Arc<str>,

    #[serde(rename = "@type")]
    type_: Option<Arc<str>>,

    #[serde(rename = "require")]
    require: Vec<VkExtensionRequireSerde>,
}

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
pub struct VkExtensionRequireSerde {
    #[serde(rename = "enum")]
    enum_members: Option<Vec<VkExtensionEnumMemberSerde>>,
}

impl VkExtensionSerde {
    #[must_use]
    pub fn fix(&self) -> Option<VkExtension> {
        Some(VkExtension {
            name: self.name.clone(),
            number: self.number.clone(),
            enum_members: self
                .require
                .iter()
                .flat_map(|require| {
                    require
                        .enum_members
                        .as_ref()
                        .map(|x| x.iter().filter_map(|x| x.fix(self)).collect::<Vec<_>>())
                })
                .flatten()
                .collect(),
        })
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct VkExtension {
    pub name: Arc<str>,
    pub number: Arc<str>,
    pub enum_members: Vec<VkExtensionEnumMember>,
}

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
struct VkExtensionEnumMemberSerde {
    #[serde(rename = "@name")]
    name: Arc<str>,

    #[serde(rename = "@api")]
    api: Option<Arc<str>>,

    #[serde(rename = "@value")]
    value: Option<Arc<str>>,

    #[serde(rename = "@offset")]
    offset: Option<Arc<str>>,

    #[serde(rename = "@extnumber")]
    extnumber: Option<Arc<str>>,

    #[serde(rename = "@bitpos")]
    bitpos: Option<Arc<str>>,

    #[serde(rename = "@extends")]
    extends: Option<Arc<str>>,

    #[serde(rename = "@alias")]
    alias: Option<Arc<str>>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkExtensionEnumMember {
    Value {
        name: Arc<str>,
        value: Arc<str>,
        extends: Option<Arc<str>>,
    },
    ExtendsOffset {
        name: Arc<str>,
        number: isize,
        extends: Arc<str>,
    },
    ExtendsBitpos {
        name: Arc<str>,
        bitpos: Arc<str>,
        extends: Arc<str>,
    },
    ExtendsAlias {
        name: Arc<str>,
        alias: Arc<str>,
        extends: Arc<str>,
    },
    Alias {
        name: Arc<str>,
        alias: Arc<str>,
    },
}

impl VkExtensionEnumMemberSerde {
    pub fn fix(&self, extension: &VkExtensionSerde) -> Option<VkExtensionEnumMember> {
        if matches!(self.api.as_deref(), Some("vulkansc"))
            || self.name.as_ref() == "VK_ANDROID_NATIVE_BUFFER_NUMBER"
        {
            None
        } else if let Some(value) = &self.value {
            Some(VkExtensionEnumMember::Value {
                name: self.name.clone(),
                value: value.clone(),
                extends: self.extends.clone(),
            })
        } else if let Some(offset) = &self.offset {
            let Some(extends) = &self.extends else {
                unreachable!()
            };
            let Ok(offset) = parse_int::parse::<isize>(offset) else {
                unreachable!()
            };
            let Ok(extension_number) = parse_int::parse::<isize>(
                self.extnumber
                    .as_ref()
                    .map_or_else(|| extension.number.as_ref(), |extnumber| extnumber.as_ref()),
            ) else {
                unreachable!()
            };
            let number = 1000000000 + (extension_number - 1) * 1000 + offset;
            Some(VkExtensionEnumMember::ExtendsOffset {
                name: self.name.clone(),
                number,
                extends: extends.clone(),
            })
        } else if let Some(bitpos) = &self.bitpos {
            let Some(extends) = &self.extends else {
                unreachable!()
            };
            Some(VkExtensionEnumMember::ExtendsBitpos {
                name: self.name.clone(),
                bitpos: bitpos.clone(),
                extends: extends.clone(),
            })
        } else if let Some(alias) = &self.alias {
            self.extends.as_ref().map_or_else(
                || {
                    Some(VkExtensionEnumMember::Alias {
                        name: self.name.clone(),
                        alias: alias.clone(),
                    })
                },
                |extends| {
                    Some(VkExtensionEnumMember::ExtendsAlias {
                        name: self.name.clone(),
                        alias: alias.clone(),
                        extends: extends.clone(),
                    })
                },
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn works_vk_xml_enums_tag_parsing() {
        let vk_xml_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vk.xml"));
        let vk_xml = VkXml::from(vk_xml_path).expect("vk_xml");
        assert!(!vk_xml.enums.is_empty());

        let _expected_api_constants = vec![
            "VK_QUEUE_FAMILY_IGNORED",
            "VK_SHADER_UNUSED_KHR",
            "VK_SHADER_UNUSED_NV",
        ];
        let mut expected_enums = vec![
            "VkOpticalFlowPerformanceLevelNV",
            "VkValidationCacheHeaderVersionEXT",
            "VkFenceImportFlagBits",
            "VkPipelineCacheCreateFlagBits",
            "VkDescriptorSetLayoutCreateFlagBits",
            "VkPipelineDepthStencilStateCreateFlagBits",
        ];
        for enums in &vk_xml.enums {
            match enums {
                VkEnums::ApiConstants { members: _ } => {}
                VkEnums::Enum { name, members: _ } => {
                    expected_enums.retain(|x| x != &name.as_ref());
                }
            };
        }
        assert!(expected_enums.is_empty());

        for enums in &vk_xml.enums {
            if let VkEnums::Enum { name: _, members } = enums {
                for member in members {
                    match member {
                        VkEnumsMember::ApiConstantMember {
                            name: _,
                            type_: _,
                            value: _,
                        }
                        | VkEnumsMember::Alias {
                            name: _,
                            alias: _,
                            enum_name: _,
                        } => continue,
                        VkEnumsMember::ApiConstantAlias { name: _, alias: _ } => {}
                        VkEnumsMember::MemberValue { name: _, value, .. } => {
                            if let Some(value) = value {
                                assert!(parse_int::parse::<isize>(value).is_ok());
                            }
                        }
                        VkEnumsMember::MemberBitpos { .. } => continue,
                    }
                }
            }
        }
    }

    #[test]
    fn works_vk_xml_type_attributes_parsing() {
        let vk_xml_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vk.xml"));
        let vk_xml = VkXml::from(vk_xml_path).expect("vk_xml");
        assert!(!vk_xml.type_attributes.is_empty());

        let mut expected = vec![(
            "VkBindBufferMemoryDeviceGroupInfoKHR",
            "VkBindBufferMemoryDeviceGroupInfo",
        )];
        for type_attribute in &vk_xml.type_attributes {
            match type_attribute {
                VkTypeAttributes::Alias { name, alias } => {
                    expected.retain(|x| *x != (name.as_ref(), alias.as_ref()));
                }
            }
        }
        assert!(expected.is_empty());
    }

    #[test]
    fn works_vk_xml_typedef_extern_parsing() {
        let vk_xml_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vk.xml"));
        let vk_xml = VkXml::from(vk_xml_path).expect("vk_xml");
        assert!(!vk_xml.typedefs.is_empty());
        assert!(!vk_xml.type_externs.is_empty());
    }
}
