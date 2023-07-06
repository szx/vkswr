mod vk_xml;

use lazy_static::lazy_static;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::sync::{Arc, Mutex};
use thiserror::Error;
pub use vk_xml::*;

impl VkXml {
    /// # Errors
    ///
    /// Will return `Err` if writing to `f` fails.
    pub fn write_decls(&self, f: &mut impl std::io::Write) -> Result<(), WriteVkXmlError> {
        for x in &self.enums {
            let tokens = x.into_token_stream();
            writeln!(f, "{tokens}")?;
        }

        for x in &self.structs {
            let tokens = x.into_token_stream();
            writeln!(f, "{tokens}")?;
        }

        for x in &self.typedefs {
            let tokens = x.into_token_stream();
            writeln!(f, "{tokens}")?;
        }

        for x in &self.type_attributes {
            let tokens = x.into_token_stream();
            writeln!(f, "{tokens}")?;
        }

        Self::add_externs();
        for x in &self.type_externs {
            let tokens = x.into_token_stream();
            writeln!(f, "{tokens}")?;
        }

        for x in &self.extensions {
            let tokens = x.into_token_stream();
            writeln!(f, "{tokens}")?;
        }

        writeln!(f, "{}", quote! {pub type int = i32;})?;

        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Err` if writing to `f` fails.
    pub fn write_defs(&self, f: &mut impl std::io::Write) -> Result<(), WriteVkXmlError> {
        writeln!(
            f,
            "#[macro_export] macro_rules! include_commands {{ () => {{"
        )?;
        for x in &self.commands {
            let tokens = x.into_token_stream();
            writeln!(f, "{tokens}")?;
        }
        writeln!(f, "}}}}")?;

        Ok(())
    }

    fn add_externs() {
        Self::add_extern("xcb_connection_t");
        Self::add_extern("xcb_window_t");
    }

    fn add_extern(name: &str) {
        seen(Arc::from(name));
    }
}

#[derive(Error, Debug)]
pub enum WriteVkXmlError {
    #[error("IO write error: {0}")]
    IO(#[from] std::io::Error),
}

lazy_static! {
    static ref SEEN: Mutex<Vec<Arc<str>>> = Mutex::new(Vec::new());
}

fn seen(name: Arc<str>) -> bool {
    let mut seen = SEEN.lock().unwrap();
    if seen.iter().any(|x| x == &name) {
        true
    } else {
        seen.push(name);
        false
    }
}

fn enum_type_from_name(name: &Arc<str>) -> &'static str {
    if name.ends_with('2') {
        "u64"
    } else {
        "u32"
    }
}
impl ToTokens for VkEnums {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::ApiConstants { members } => {
                let aliases = members.iter();
                tokens.extend(quote!(#(#aliases)*).to_token_stream());
            }
            Self::Enum { name, members } => {
                let type_ = format_ident!("{}", enum_type_from_name(name));
                let name = format_ident!("{}", name.as_ref());
                let mut enumerators = members.iter().filter(|x| {
                    matches!(x, VkEnumsMember::MemberValue { .. })
                        || matches!(x, VkEnumsMember::MemberBitpos { .. })
                });

                tokens.extend(quote! {
                    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
                    #[repr(transparent)]
                    pub struct #name(pub #type_);
                    impl #name {
                        #(#enumerators)*
                    }
                    impl std::ops::BitOr for #name {
                        type Output = Self;
                        #[inline]
                        fn bitor(self, rhs: Self) -> Self::Output {
                            Self(self.0 | rhs.0)
                        }
                    }
                    impl std::ops::BitAnd for #name {
                        type Output = Self;
                        #[inline]
                        fn bitand(self, rhs: Self) -> Self::Output {
                            Self(self.0 & rhs.0)
                        }
                    }
                    impl PartialEq<#type_> for #name {
                        fn eq(&self, other: &#type_) -> bool { self.0 == *other }
                    }
                    impl std::convert::From<#name> for #type_ {
                        #[inline]
                        fn from(value: #name) -> Self {
                            value.0
                        }
                    }
                    impl std::convert::From<#type_> for #name {
                        #[inline]
                        fn from(value: #type_) -> Self {
                            Self(value)
                        }
                    }
                });

                let aliases = members
                    .iter()
                    .filter(|x| matches!(x, VkEnumsMember::Alias { .. }));
                tokens.extend(quote! {
                    impl #name {
                        #(pub const #aliases;)*
                    }
                });
            }
        }
    }
}

impl ToTokens for VkEnumsMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::ApiConstantMember { name, type_, value } => {
                let name = format_ident!("{}", name.as_ref());
                let type_ = type_.0.parse::<TokenStream>().expect("Rust type");
                let value = value.parse::<TokenStream>().expect("Rust value");
                tokens.extend(quote!(pub const #name : #type_ = #value;).to_token_stream());
                let type_name = format_ident!("_{}_type", name);
                tokens.extend(quote!(pub type #type_name = #type_;).to_token_stream());
            }
            Self::ApiConstantAlias { name, alias } => {
                let name = format_ident!("{}", name.as_ref());
                let type_name = format_ident!("_{}_type", alias.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                tokens.extend(quote!(pub const #name : #type_name = #alias;).to_token_stream());
            }
            Self::MemberValue {
                name,
                value,
                enum_name,
            } => {
                let name = format_ident!("{}", name.as_ref());
                if let Some(value) = value {
                    let value = if enum_type_from_name(enum_name) == "u64" {
                        Literal::i64_suffixed(parse_int::parse(value).expect("integer"))
                    } else {
                        Literal::i32_suffixed(parse_int::parse(value).expect("integer"))
                    };
                    tokens.extend(
                        // TODO: use bytemuck instead of transmute
                        quote! { pub const #name : Self = Self(unsafe { std::mem::transmute(#value) }); },
                    );
                } else {
                    tokens.extend(quote! { #name,});
                }
            }
            Self::MemberBitpos {
                name,
                bitpos,
                enum_name,
            } => {
                let name = format_ident!("{}", name.as_ref());
                let bitpos = Literal::isize_unsuffixed(parse_int::parse(bitpos).expect("integer"));
                tokens.extend(quote! {  pub const #name : Self = Self(1 << #bitpos); });
            }
            Self::Alias { name, alias, .. } => {
                let name = format_ident!("{}", name.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                tokens.extend(quote!(#name: Self = Self::#alias).to_token_stream());
            }
        }
    }
}

impl ToTokens for VkStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Alias { name, alias } => {
                let name = format_ident!("{}", name.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                tokens.extend(quote!(pub type #name = #alias;).to_token_stream());
            }
            Self::Struct { name, members } => {
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {
                    #[repr(C)]
                    #[derive(Debug, Copy, Clone)]
                    pub struct #name {
                        #(#members)*
                    }
                });
            }
            Self::Union { name, members } => {
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {
                    #[repr(C)]
                    #[derive(Copy, Clone)]
                    pub union #name {
                        #(#members)*
                    }
                    impl std::fmt::Debug for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.debug_struct("#name")
                                .finish()
                        }
                    }
                });
            }
        }
    }
}

impl ToTokens for VkStructMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Member {
                name,
                type_,
                in_union,
            } => {
                let name = format_ident!("{}", name.as_ref());
                let type_ = if *in_union {
                    format!("std::mem::ManuallyDrop<{}>", type_.0)
                } else {
                    type_.0.clone()
                };
                let type_ = type_.parse::<TokenStream>().expect("Rust type");
                tokens.extend(quote!(pub #name : #type_,).to_token_stream());
            }
        }
    }
}

impl ToTokens for VkTypedef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Alias { name, type_ } => {
                let name = format_ident!("{}", name.as_ref());
                let type_ = type_.0.parse::<TokenStream>().expect("Rust type");
                tokens.extend(quote!(pub type #name = #type_;).to_token_stream());
            }
        }
    }
}

impl ToTokens for VkTypeAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Alias { name, alias } => {
                let name = format_ident!("{}", name.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                tokens.extend(quote! {pub type #name = #alias ;});
            }
        }
    }
}

impl ToTokens for VkTypeExtern {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Extern { name } => {
                if seen(name.clone()) {
                    return;
                }
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {pub type #name = VkUnsupportedType;});
            }
            Self::FuncPointer {
                type_,
                name,
                members,
            } => {
                let name = format_ident!("{}", name.as_ref());
                if let Some(type_) = type_ {
                    let type_ = type_.0.parse::<TokenStream>().expect("Rust type");
                    tokens.extend(
                        quote! {pub type #name = Option<unsafe extern "C" fn(#(#members)*) -> #type_>;},
                    );
                } else {
                    tokens.extend(
                        quote! {pub type #name = Option<unsafe extern "C" fn(#(#members)*)>;},
                    );
                }
            }
        }
    }
}

impl ToTokens for VkFuncDeclMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Member { name, type_ } => {
                let name = format_ident!("{}", name);
                let type_ = type_.0.parse::<TokenStream>().expect("Rust type");
                tokens.extend(quote!(#name: #type_,).to_token_stream());
            }
        }
    }
}

impl VkFuncDeclMember {
    fn to_arg(&self) -> Ident {
        match self {
            Self::Member { name, .. } => {
                format_ident!("{}", name)
            }
        }
    }
}

impl ToTokens for VkCommand {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Command {
                type_,
                name,
                members,
            } => {
                let command = format_ident!("icd_{}", name.as_ref());
                let args = members
                    .iter()
                    .map(VkFuncDeclMember::to_arg)
                    .collect::<Vec<_>>();
                let name = format_ident!("{}", name.as_ref());
                if let Some(type_) = type_ {
                    let type_ = type_.0.parse::<TokenStream>().expect("Rust type");
                    tokens.extend(quote! {#[no_mangle] pub unsafe  extern "C" fn #name(#(#members)*) -> #type_ {#command(#(#args,)*)}});
                } else {
                    tokens.extend(
                        quote! {#[no_mangle] pub unsafe extern "C" fn #name(#(#members)*) {#command(#(#args,)*)}},
                    );
                }
            }
        }
    }
}

impl ToTokens for VkExtension {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VkExtension {
            name,
            number,
            enum_members,
        } = self;
        {
            if !(name.contains("VK_VERSION") || name.contains("VKSC_VERSION")) {
                let number_name = format_ident!("{}_NUMBER", name.to_uppercase());
                let number = number.parse::<TokenStream>().expect("Rust value");
                tokens.extend(quote! {pub const #number_name : u64 = #number;});
            }

            tokens.extend(quote! {#(#enum_members)*});
        }
    }
}

impl ToTokens for VkExtensionEnumMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let get_type_by_name = |name: &Arc<str>| -> TokenStream {
            if name.ends_with("_NAME") {
                "&str"
            } else {
                "isize"
            }
            .parse::<TokenStream>()
            .expect("Rust value")
        };
        let get_type_by_value = |value: &Arc<str>| -> TokenStream {
            if value.starts_with('"') {
                "&str"
            } else {
                "isize"
            }
            .parse::<TokenStream>()
            .expect("Rust value")
        };
        match self {
            Self::Value {
                name,
                value,
                extends,
            } => {
                if seen(name.clone()) {
                    return;
                }
                let type_ = get_type_by_value(value);
                let name = format_ident!("{}", name.as_ref());
                let value = value.parse::<TokenStream>().expect("Rust value");
                if let Some(extends) = extends {
                    let extends = format_ident!("{}", extends.as_ref());
                    tokens.extend(quote! {
                        impl #extends {
                           pub const #name : Self = Self(#value);
                        }
                    });
                } else {
                    tokens.extend(quote! {pub const #name : #type_ = #value;});
                }
            }
            Self::Alias { name, alias } => {
                if seen(name.clone()) {
                    return;
                }
                let type_ = get_type_by_name(name);
                let name = format_ident!("{}", name.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                tokens.extend(quote! { pub const #name : #type_ = #alias; });
            }
            Self::ExtendsOffset {
                name,
                number,
                extends,
            } => {
                if seen(name.clone()) {
                    return;
                }
                let name = format_ident!("{}", name.as_ref());
                let number = Literal::isize_unsuffixed(*number);
                let extends = format_ident!("{}", extends.as_ref());
                tokens.extend(quote! {
                    impl #extends {
                       pub const #name : Self = Self(#number);
                    }
                });
            }
            Self::ExtendsBitpos {
                name,
                bitpos,
                extends,
            } => {
                if seen(name.clone()) {
                    return;
                }
                let name = format_ident!("{}", name.as_ref());
                let bitpos = Literal::isize_unsuffixed(parse_int::parse(bitpos).expect("integer"));
                let extends = format_ident!("{}", extends.as_ref());
                tokens.extend(quote! {
                    impl #extends {
                       pub const #name : Self = Self(1 << #bitpos);
                    }
                });
            }
            Self::ExtendsAlias {
                name,
                alias,
                extends,
            } => {
                if seen(name.clone()) {
                    return;
                }
                let name = format_ident!("{}", name.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                let extends = format_ident!("{}", extends.as_ref());
                tokens.extend(quote! {
                    impl #extends {
                       pub const #name : Self = Self::#alias;
                    }
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    use std::path::PathBuf;

    #[test]
    fn works_to_tokens() {
        let vk_xml_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vk.xml"));
        let vk_xml = VkXml::from(vk_xml_path).expect("vk_xml");
        vk_xml.write_decls(&mut io::sink()).expect("write succeeds");
        vk_xml.write_defs(&mut io::sink()).expect("write succeeds");
    }
}
