mod vk_xml;

use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
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

        for x in &self.type_externs {
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
        writeln!(f, "#[macro_export] macro_rules! include_commands {{ () => {{")?;
        for x in &self.commands {
            let tokens = x.into_token_stream();
            writeln!(f, "{tokens}")?;
        }
        writeln!(f, "}}}}")?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum WriteVkXmlError {
    #[error("IO write error: {0}")]
    IO(#[from] std::io::Error),
}

impl ToTokens for VkEnums {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::ApiConstants { members } => {
                let aliases = members.iter();
                tokens.extend(quote!(#(#aliases)*).to_token_stream());
            }
            Self::Enum { name, members } => {
                let name = format_ident!("{}", name.as_ref());
                let enumerators = members
                    .iter()
                    .filter(|x| matches!(x, VkEnumsMember::Member { .. }));
                tokens.extend(quote! {
                    #[derive(Debug, Eq, PartialEq)]
                    #[repr(C)]
                    pub enum #name {
                        #(#enumerators)*
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
            Self::Bitmask { name } => {
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {pub type #name = VkFlags ;});
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
            Self::Member { name, value } => {
                let name = format_ident!("{}", name.as_ref());
                if let Some(value) = value {
                    let value = Literal::isize_suffixed(parse_int::parse(value).expect("integer"));
                    tokens.extend(quote! { #name = #value, });
                } else {
                    tokens.extend(quote! { #name,});
                }
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
                    #[derive(Debug)]
                    pub struct #name {
                        #(#members)*
                    }
                });
            }
            Self::Union { name, members } => {
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {
                    #[repr(C)]
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
        vk_xml.write_defs(&mut io::stdout()).expect("write succeeds");
    }
}
