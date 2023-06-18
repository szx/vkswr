use lazy_static::lazy_static;
use phf::phf_map;
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
    pub enums: HashSet<VkXmlEnums>,
    pub structs: HashSet<VkStruct>,
    pub typedefs: HashSet<VkTypedef>,
    pub type_attributes: HashSet<VkTypeAttributes>,
    pub type_externs: HashSet<VkTypeExtern>,
}

impl VkXml {
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
    let mut vk_xml: VkXml = Default::default();

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
                        if let Ok(mut e) = quick_xml::de::from_str::<VkXmlEnumsSerde>(str) {
                            vk_xml.enums.insert(e.fix());
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
                        if let Ok(mut x) = quick_xml::de::from_str::<VkTypedefSerde>(str) {
                            if let Some(x) = x.fix() {
                                vk_xml.typedefs.insert(x);
                            }
                            continue;
                        }
                        if let Ok(mut x) = quick_xml::de::from_str::<VkTypeAttributesSerde>(str) {
                            vk_xml.type_attributes.insert(x.fix());
                            continue;
                        }
                        if let Ok(mut x) = quick_xml::de::from_str::<VkTypeExternSerde>(str) {
                            vk_xml.type_externs.insert(x.fix());
                            continue;
                        }
                        if let Ok(mut x) = quick_xml::de::from_str::<VkTypeExternSerde2>(str) {
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
                        // TODO: Parse functions
                    }
                    _ => (),
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
    pub fn fix(&self) -> VkXmlEnums {
        let Some(type_) = &self.type_ else {
            return VkXmlEnums::ApiConstants {
                members: self
                    .enumerators
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|x| x.fix(self.name.clone()))
                    .collect(),
            };
        };
        let type_ = type_.as_ref();
        if type_ == "enum" || type_ == "bitmask" && self.enumerators.is_some() {
            VkXmlEnums::Enum {
                name: self.name.clone(),
                members: self
                    .enumerators
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|x| x.fix(self.name.clone()))
                    .collect(),
            }
        } else if type_ == "bitmask" && self.enumerators.is_none() {
            VkXmlEnums::Bitmask {
                name: self.name.clone(),
            }
        } else {
            unreachable!("{:?}", self)
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkXmlEnums {
    ApiConstants {
        members: Vec<VkXmlEnumsMember>,
    },
    Enum {
        name: Arc<str>,
        members: Vec<VkXmlEnumsMember>,
    },
    Bitmask {
        name: Arc<str>,
    },
}

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
struct VkXmlEnumsMemberSerde {
    #[serde(rename = "@name")]
    name: Arc<str>,

    #[serde(rename = "@value")]
    value: Option<Arc<str>>,

    #[serde(rename = "@type")]
    type_: Option<Arc<str>>,

    #[serde(rename = "@alias")]
    alias: Option<Arc<str>>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkXmlEnumsMember {
    Member {
        name: Arc<str>,
        value: Option<Arc<str>>,
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
    pub fn fix(&self, enum_name: Arc<str>) -> VkXmlEnumsMember {
        if enum_name.as_ref() == "API Constants" {
            if let Some(alias) = &self.alias {
                VkXmlEnumsMember::ApiConstantAlias {
                    name: self.name.clone(),
                    alias: alias.clone(),
                }
            } else {
                VkXmlEnumsMember::ApiConstantMember {
                    name: self.name.clone(),
                    type_: VkFFIType::new(self.type_.clone().unwrap().as_ref()),
                    value: VkFFIType::parse_value(self.value.as_ref().unwrap().clone()),
                }
            }
        } else if let Some(alias) = &self.alias {
            VkXmlEnumsMember::Alias {
                name: self.name.clone(),
                alias: alias.clone(),
                enum_name,
            }
        } else {
            VkXmlEnumsMember::Member {
                name: self.name.clone(),
                value: self.value.clone(),
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
            "include" => None,
            "define" => None,
            "enum" if self.alias.is_none() => None,
            "bitmask" | "handle" | "enum" => {
                let alias = self.alias.as_ref().unwrap().clone();
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
            static ref re_extract_member: regex::Regex =
                regex::Regex::new(r"<member[^>]*>(.*?)</member[^>]*>").unwrap();
            static ref re_extract_name: regex::Regex =
                regex::Regex::new(r"<name[^>]*>(.*?)</name[^>]*>").unwrap();
            static ref re_remove_comments: regex::Regex =
                regex::Regex::new(r"<comment[^>]*>(.*?)</comment[^>]*>").unwrap();
            static ref re_remove_tags: regex::Regex = regex::Regex::new(r"</?[^>]*>").unwrap();
            static ref re_replace_spaces: regex::Regex = regex::Regex::new(r"\s\s+").unwrap();
        }
        let mut members = vec![];
        for cap in re_extract_member.captures_iter(str) {
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
            let mut member = VkStructMemberSerde { type_, name };
            members.push(member);
        }

        let mut iter = members.iter_mut();
        let mut members: Vec<VkStructMember> = vec![];
        while let Some(member) = iter.next() {
            if member.is_bitfield_24() {
                let next = iter.next();
                members.push(member.fix(next, &is_union));
            } else {
                members.push(member.fix(None, &is_union));
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

    fn is_bitfield_8(&self) -> bool {
        self.type_.contains(" :8")
    }

    fn fix(&mut self, next: Option<&mut VkStructMemberSerde>, in_union: &bool) -> VkStructMember {
        lazy_static! {
            static ref re_bitfield_24: regex::Regex = regex::Regex::new(r"(.*?)\s\:24").unwrap();
        }

        if let Some(cap) = re_bitfield_24.captures(&self.type_) {
            self.type_ = cap.get(1).unwrap().as_str().into();
            if let Some(next) = next {
                self.name = format!("{}_{}", self.name, next.name);
            }
        }

        let name = self.name.clone().into();
        let type_ = VkFFIType::new(&self.type_);

        let in_union = *in_union;
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

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum VkFuncDeclMember {
    Member { name: String, type_: VkFFIType },
}

impl VkFuncDeclMember {
    fn parse_members(str: &str) -> Option<VkTypeExtern> {
        lazy_static! {
            static ref re_remove_tags: regex::Regex = regex::Regex::new(r"</?[^>]*>").unwrap();
            static ref re_replace_spaces: regex::Regex = regex::Regex::new(r"\s\s+").unwrap();
            static ref re_type_name_members: regex::Regex = regex::Regex::new(
                r"typedef\s(.*?)\s\(VKAPI_PTR\s\*\s(.*?)\s\)\((?:\s|void)(.*?)\);"
            )
            .unwrap();
            static ref re_member: regex::Regex =
                regex::Regex::new(r"\s?(.*?)\s(\w*?)[,)]").unwrap();
        }

        let str = re_remove_tags.replace_all(&str, " ");
        let str = re_replace_spaces.replace_all(&str, " ");
        if str.contains("PFN_vkVoidFunction") {
            dbg!(&str);
        }
        let Some(cap) = re_type_name_members.captures(str.as_ref()) else { return None };

        let type_ = cap.get(1).unwrap().as_str();
        let type_ = if type_ == "void" {
            None
        } else {
            Some(VkFFIType::new(type_))
        };

        let name = cap.get(2).unwrap().as_str().into();

        let raw_members = cap.get(3).unwrap().as_str();
        let mut members: Vec<VkFuncDeclMember> = vec![];
        for cap in re_member.captures_iter(raw_members) {
            let type_ = VkFFIType::new(cap[1].into());
            let name = cap[2].into();
            members.push(VkFuncDeclMember::Member { name, type_ });
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
            static ref re_replace_struct: regex::Regex = regex::Regex::new(r"struct\s").unwrap();
            static ref re_const_ptr: regex::Regex = regex::Regex::new(r"const\s(.*?)\s\*").unwrap();
            static ref re_mut_ptr: regex::Regex = regex::Regex::new(r"(.*?)\s?\*").unwrap();
            static ref re_array: regex::Regex = regex::Regex::new(r"(.*?)\s\[(.*?)\]").unwrap();
        }

        let mut type_: String = str.to_string();
        type_ = re_replace_struct.replace_all(&type_, "").into();
        assert!(!type_.contains(':'), "{:?}", str);

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
        let ffi = C_TYPE_TO_FFI.get(&type_);

        if let Some(ffi) = ffi {
            type_.replace_range(.., ffi);
        }
        if let Some(array_index) = array_index {
            type_ = format!("[{}; {} as usize]", type_, array_index);
        }
        if is_const_ptr {
            type_ = format!("*const {}", type_);
        } else if is_mut_ptr {
            type_ = format!("*mut {}", type_);
        }

        Self { 0: type_ }
    }

    fn parse_value(value: Arc<str>) -> String {
        if let Ok(value) = parse_int::parse::<usize>(value.as_ref()) {
            format!("{}", value)
        } else if value.ends_with('F') {
            format!("{}_f32", value.split('F').next().unwrap())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn works_vk_xml_enums_tag_parsing() {
        let vk_xml_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vk.xml"));
        let vk_xml = VkXml::from(vk_xml_path).expect("vk_xml");
        assert!(!vk_xml.enums.is_empty());

        let mut expected_api_constants = vec![
            "VK_QUEUE_FAMILY_IGNORED",
            "VK_SHADER_UNUSED_KHR",
            "VK_SHADER_UNUSED_NV",
        ];
        let mut expected_enums = vec![
            "VkOpticalFlowPerformanceLevelNV",
            "VkValidationCacheHeaderVersionEXT",
            "VkFenceImportFlagBits",
        ];
        let mut expected_bitmasks = vec![
            "VkPipelineCacheCreateFlagBits",
            "VkDescriptorSetLayoutCreateFlagBits",
            "VkPipelineDepthStencilStateCreateFlagBits",
        ];
        for enums in &vk_xml.enums {
            match enums {
                VkXmlEnums::ApiConstants { members } => {}
                VkXmlEnums::Enum { name, members } => {
                    expected_enums.retain(|x| x != &name.as_ref())
                }
                VkXmlEnums::Bitmask { name } => expected_bitmasks.retain(|x| x != &name.as_ref()),
            };
        }
        assert!(expected_enums.is_empty());
        assert!(expected_bitmasks.is_empty());

        for enums in &vk_xml.enums {
            if let VkXmlEnums::Enum { name, members } = enums {
                assert!(!members.is_empty());
                for member in members {
                    match member {
                        VkXmlEnumsMember::ApiConstantMember { name, type_, value } => {}
                        VkXmlEnumsMember::ApiConstantAlias { name, alias } => {}
                        VkXmlEnumsMember::Member { name, value } => {
                            if let Some(value) = value {
                                assert!(parse_int::parse::<isize>(value).is_ok());
                            }
                        }
                        VkXmlEnumsMember::Alias {
                            name,
                            alias,
                            enum_name,
                        } => {}
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
                    expected.retain(|x| *x != (name.as_ref(), alias.as_ref()))
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
        dbg!(&vk_xml.type_externs);
    }
}
