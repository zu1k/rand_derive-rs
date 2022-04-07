extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Rand)]
pub fn rand_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let tokens = impl_rand_derive(&ast);
    TokenStream::from(tokens)
}

fn impl_rand_derive(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let rand = match ast.data {
        syn::Data::Struct(ref data) => {
            let fields = data
                .fields
                .iter()
                .filter_map(|field| field.ident.as_ref())
                .map(|ident| quote! { #ident: __rng.gen() })
                .collect::<Vec<_>>();

            quote! { #name { #(#fields,)* } }
        }
        syn::Data::Enum(ref data) => {
            let ref virants = data.variants;
            if virants.is_empty() {
                panic!("`Rand` cannot be derived for enums with no variants");
            }

            let len = virants.len();
            let mut arms = virants.iter().map(|variant| {
                let ref ident = variant.ident;
                match &variant.fields {
                    syn::Fields::Named(field) => {
                        let fields = field
                            .named
                            .iter()
                            .filter_map(|field| field.ident.as_ref())
                            .map(|ident| quote! { #ident: __rng.gen() })
                            .collect::<Vec<_>>();
                        quote! { #name::#ident { #(#fields,)* } }
                    }
                    syn::Fields::Unnamed(field) => {
                        let fields = field
                            .unnamed
                            .iter()
                            .map(|field| {
                                if inner_type_is_vec(&field.ty) {
                                    quote! { 
                                        {
                                            let i = __rng.gen_range(0..100);
                                            __rng.sample_iter(::rand::distributions::Standard).take(i).collect()
                                        }
                                        }
                                } else {
                                    quote! { __rng.gen() }
                                }
                            })
                            .collect::<Vec<_>>();
                        quote! { #name::#ident (#(#fields),*) }
                    }
                    syn::Fields::Unit => quote! { #name::#ident },
                }
            });

            match len {
                1 => quote! { #(#arms)* },
                2 => {
                    let (a, b) = (arms.next(), arms.next());
                    quote! { if __rng.gen() { #a } else { #b } }
                }
                _ => {
                    let mut variants = arms
                        .enumerate()
                        .map(|(index, arm)| quote! { #index => #arm })
                        .collect::<Vec<_>>();
                    variants.push(quote! { _ => unreachable!() });
                    quote! { match __rng.gen_range(0..#len) { #(#variants,)* } }
                }
            }
        }
        _ => unimplemented!(),
    };

    quote! {
        impl #impl_generics ::rand::distributions::Distribution<#name>
            for ::rand::distributions::Standard
            #ty_generics
            #where_clause
        {
            #[inline]
            fn sample<__R: ::rand::Rng + ?Sized>(&self, __rng: &mut __R) -> #name {
                #rand
            }
        }
    }
}

fn inner_type_is_vec(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        if let Some(seg) = path.segments.last() {
            return seg.ident == "Vec"
        }
    }
    false
}
