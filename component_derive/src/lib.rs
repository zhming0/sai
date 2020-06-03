extern crate proc_macro;
use proc_macro::TokenStream;
#[macro_use]
extern crate syn;
use syn::{DeriveInput, Data, Field};
use syn::Meta::{Path, NameValue, List};
use syn::NestedMeta::{Lit, Meta};
use syn::export::{TokenStream2};
#[macro_use]
extern crate quote;

mod symbol;
use symbol::*;


#[proc_macro_derive(Component, attributes(injected))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // println!("{}", ast.attrs);
    let ident = ast.ident;

    let fields = parse_struct_data(&ast.data);

    let fields_tokens = build_struct_fields(&fields);

    let tokens = quote!{
        impl shine::Component for #ident {
             fn build(registry: &shine::ComponentRepository) -> #ident {
                return #ident {
                    #fields_tokens
                }

            }
        }
    };

    return tokens.into();
}

fn build_struct_fields (fields: &Vec<ComponentField>) -> TokenStream2 {

    let x: Vec<TokenStream2> = fields
        .into_iter()
        .map(|f| {
            let ident = &f.ident;
            quote! {
                #ident: Default::default()
            }
        })
        .collect();


    quote!{
        #(#x),*
    }
}


struct ComponentField {
    injected: bool,
    ident: syn::Ident,
    ty: syn::Type
}

fn parse_struct_data (data: &Data) -> Vec<ComponentField> {

    let s = match data {
        Data::Struct(s) => s,
        _ => panic!("Component macro can only be used on struct enum")
    };


    let fields = match &s.fields {
        syn::Fields::Named(f) => f,
        syn::Fields::Unit => return Vec::new(),
        _ => panic!("Component marco can not be used on tuple struct")
    };
    let fields = &fields.named;

    return fields
        .iter()
        .map(parse_struct_field)
        .collect::<Vec<ComponentField>>();
}

fn parse_struct_field (field: &Field) -> ComponentField {

    let mut injected = false;
    let ty = field.ty.clone();
    let ident = field.ident.clone().unwrap();
    let attrs = &field.attrs;


    return ComponentField {
        injected: false,
        ident,
        ty
    }
}

fn get_injected_meta_items(attr: &syn::Attribute) -> Result<Vec::<syn::NestedMeta>, ()> {
    if attr.path != INJECTED {
        return Ok(Vec::new())
    }

    match attr.parse_meta() {
        Ok(List(meta)) => Ok(meta.nested.into_iter().collect()),
        Ok(_) => Ok(Vec::new()),
        Err(_) => Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

