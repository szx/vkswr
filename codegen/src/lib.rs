mod vk_xml;

use proc_macro2::{Literal, Punct, Spacing, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
pub use vk_xml::*;

impl Display for VkXml {
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
        for x in &self.type_attributes {
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
                dbg!(&members);
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
                    enum #name {
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
                tokens.extend(quote! {type #name = VkFlags ;});
            }
        }
    }
}

impl ToTokens for VkXmlEnumsMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VkXmlEnumsMember::ApiConstantMember { name, type_, value } => {
                dbg!(&self);
                let name = format_ident!("{}", name.as_ref());
                let type_ = type_.0.parse::<TokenStream>().unwrap();
                let value = value.parse::<TokenStream>().unwrap();
                tokens.extend(quote!(const #name : #type_ = #value;).to_token_stream());
                let type_name = format_ident!("_{}_type", name);
                tokens.extend(quote!(type #type_name = #type_;).to_token_stream());
            }
            VkXmlEnumsMember::ApiConstantAlias { name, alias } => {
                let name = format_ident!("{}", name.as_ref());
                let type_name = format_ident!("_{}_type", alias.as_ref());
                let alias = format_ident!("{}", alias.as_ref());
                tokens.extend(quote!(const #name : #type_name = #alias;).to_token_stream());
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
                tokens.extend(quote!(type #name = #alias;).to_token_stream());
            }
            VkStruct::Struct { name, members } => {
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {
                    #[repr(C)]
                    struct #name {
                        #(#members)*
                    }
                });
            }
            VkStruct::Union { name, members } => {
                let name = format_ident!("{}", name.as_ref());
                tokens.extend(quote! {
                    #[repr(C)]
                    union #name {
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
                tokens.append(format_ident!("{}", name.as_ref()));
                tokens.append(Punct::new(':', Spacing::Alone));
                let type_ = if *in_union {
                    format!("std::mem::ManuallyDrop<{}>", type_.0)
                } else {
                    type_.0.clone()
                };
                tokens.extend(type_.parse::<TokenStream>());
                tokens.append(Punct::new(',', Spacing::Alone));
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
                tokens.extend(quote!(type #name = #type_;).to_token_stream());
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
                tokens.extend(quote! {type #name = #alias ;});
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
        writeln!(io::sink(), "vk_xml: {}", vk_xml).unwrap();
    }
}
