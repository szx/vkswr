mod vk_xml;

use proc_macro2::{Ident, Literal, Punct, Spacing, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
pub use vk_xml::*;

impl VkXml {
    pub fn write_decls(&self, f: &mut impl std::io::Write) -> std::fmt::Result {
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

        for x in &self.type_attributes {
            let tokens = x.into_token_stream();
            writeln!(f, "{}", tokens).unwrap();
        }

        for x in &self.type_externs {
            let tokens = x.into_token_stream();
            writeln!(f, "{}", tokens).unwrap();
        }
        writeln!(f, "{}", quote! {pub type int = i32;}).unwrap();

        Ok(())
    }

    pub fn write_impls(&self, f: &mut impl std::io::Write) -> std::fmt::Result {
        for x in &self.commands {
            let tokens = x.into_token_stream();
            writeln!(f, "{}", tokens).unwrap();
        }

        Ok(())
    }
}

impl ToTokens for VkXmlEnums {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VkXmlEnums::ApiConstants { members } => {
                let aliases = members.iter();
                tokens.extend(quote!(#(#aliases)*).to_token_stream());
            }
            VkXmlEnums::Enum { name, members } => {
                let name = format_ident!("{}", name.as_ref());
                let enumerators = members
                    .iter()
                    .filter(|x| matches!(x, VkXmlEnumsMember::Member { .. }));
                tokens.extend(quote! {
                    #[repr(C)]
                    pub enum #name {
                        #(#enumerators)*
                    }
                });
                let aliases = members
                    .iter()
                    .filter(|x| matches!(x, VkXmlEnumsMember::Alias { .. }));
                tokens.extend(quote! {
                    impl #name {
                        #(pub const #aliases;)*
                    }
                });
            }
            VkXmlEnums::Bitmask { name } => {
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {pub type #name = VkFlags ;});
            }
        }
    }
}

impl ToTokens for VkXmlEnumsMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VkXmlEnumsMember::ApiConstantMember { name, type_, value } => {
                let name = format_ident!("{}", name.as_ref());
                let type_ = type_.0.parse::<TokenStream>().unwrap();
                let value = value.parse::<TokenStream>().unwrap();
                tokens.extend(quote!(pub const #name : #type_ = #value;).to_token_stream());
                let type_name = format_ident!("_{}_type", name);
                tokens.extend(quote!(pub type #type_name = #type_;).to_token_stream());
            }
            VkXmlEnumsMember::ApiConstantAlias { name, alias } => {
                let name = format_ident!("{}", name.as_ref());
                let type_name = format_ident!("_{}_type", alias.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                tokens.extend(quote!(pub const #name : #type_name = #alias;).to_token_stream());
            }
            VkXmlEnumsMember::Member { name, value } => {
                let name = format_ident!("{}", name.as_ref());
                if let Some(value) = value {
                    let value = Literal::isize_suffixed(parse_int::parse(value).unwrap());
                    tokens.extend(quote! { #name = #value, });
                } else {
                    tokens.extend(quote! { #name,});
                }
            }
            VkXmlEnumsMember::Alias {
                name,
                alias,
                enum_name,
            } => {
                let enum_name = format_ident!("{}", enum_name.as_ref());
                let name = format_ident!("{}", name.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                tokens.extend(quote!(#name: #enum_name = #enum_name::#alias).to_token_stream());
            }
        }
    }
}

impl ToTokens for VkStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VkStruct::Alias { name, alias } => {
                let name = format_ident!("{}", name.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                tokens.extend(quote!(pub type #name = #alias;).to_token_stream());
            }
            VkStruct::Struct { name, members } => {
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {
                    #[repr(C)]
                    pub struct #name {
                        #(#members)*
                    }
                });
            }
            VkStruct::Union { name, members } => {
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {
                    #[repr(C)]
                    pub union #name {
                        #(#members)*
                    }
                });
            }
        }
    }
}

impl ToTokens for VkStructMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VkStructMember::Member {
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
                let type_ = type_.parse::<TokenStream>().unwrap();
                tokens.extend(quote!(pub #name : #type_,).to_token_stream());
            }
        }
    }
}

impl ToTokens for VkTypedef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VkTypedef::Alias { name, type_ } => {
                let name = format_ident!("{}", name.as_ref());
                let type_ = type_.0.parse::<TokenStream>().unwrap();
                tokens.extend(quote!(pub type #name = #type_;).to_token_stream());
            }
        }
    }
}

impl ToTokens for VkTypeAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VkTypeAttributes::Alias { name, alias } => {
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
            VkTypeExtern::Extern { name } => {
                let name = format_ident!("{}", name.as_ref());
                // TODO: Smarter unusable types.
                tokens.extend(quote! {pub type #name = Box<[u8]>;});
            }
            VkTypeExtern::FuncPointer {
                type_,
                name,
                members,
            } => {
                let name = format_ident!("{}", name.as_ref());
                if let Some(type_) = type_ {
                    let type_ = type_.0.parse::<TokenStream>().unwrap();
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
            VkFuncDeclMember::Member { name, type_ } => {
                let name = format_ident!("{}", name);
                let type_ = type_.0.parse::<TokenStream>().unwrap();
                tokens.extend(quote!(#name: #type_,).to_token_stream());
            }
        }
    }
}

impl VkFuncDeclMember {
    fn to_arg(&self) -> Ident {
        match self {
            VkFuncDeclMember::Member { name, type_ } => {
                format_ident!("{}", name)
            }
        }
    }
}

impl ToTokens for VkCommand {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VkCommand::Command {
                type_,
                name,
                members,
            } => {
                let command = format_ident!("impl_{}", name.as_ref());
                let args = members.iter().map(|x| x.to_arg()).collect::<Vec<_>>();
                let name = format_ident!("{}", name.as_ref());
                if let Some(type_) = type_ {
                    let type_ = type_.0.parse::<TokenStream>().unwrap();
                    tokens.extend(quote! {#[no_mangle] pub extern "C" fn #name(#(#members)*) -> #type_ {#command(#(#args,)*)}});
                } else {
                    tokens.extend(
                        quote! {#[no_mangle] pub extern "C" fn #name(#(#members)*) {#command(#(#args,)*)}},
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn works_to_tokens() {
        let vk_xml_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vk.xml"));
        let vk_xml = VkXml::from(vk_xml_path).expect("vk_xml");
        vk_xml.write_decls(&mut io::sink()).unwrap();
    }
}
