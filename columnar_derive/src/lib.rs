#![recursion_limit="128"]

#![cfg(not(test))]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

#[cfg(feature = "verbose")]
extern crate rustfmt;

use proc_macro::TokenStream;
use syn::Ident;

#[proc_macro_derive(Columnar)]
pub fn derive_columnar(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).expect("Couldn't parse source");

    let result = match ast.body {
        syn::Body::Enum(_) => panic!("Enum not supported!"),
        syn::Body::Struct(ref variant_data) => {
            let columar_data = ColumnarData::new(&ast, &variant_data);
            columar_data.columnar_struct()
        }
    };

    let result_string = result.to_string();
    if cfg!(feature = "verbose") {
        print_generated_code(result_string);
    }
    result.to_string().parse().unwrap()
}

#[cfg(feature = "verbose")]
fn print_generated_code(result_string: String) {
    // Use rustfmt for pretty-printing
    let input = rustfmt::Input::Text(result_string);
    let config = rustfmt::config::Config::default();
    let (error_summary, file_map, _report) = rustfmt::format_input::<std::io::Stdout>(input, &config, None)
        .unwrap();
    assert!(error_summary.has_no_errors());
    for &(ref file_name, ref text) in &file_map {
        if file_name == "stdin" {
            println!("Formatted source:\n{}", text.to_string());
            break;
        }
    }
}

#[cfg(not(feature = "verbose"))]
fn print_generated_code(result_string: &String) {
    // Nop
}
struct ColumnarData<'a> {
    type_ref: Ident,
    type_ref_mut: Ident,
    type_columnar: Ident,
    type_iter: Ident,
    type_iter_mut: Ident,

    ast: &'a syn::MacroInput,
    fields: &'a [syn::Field],
}

impl<'a> ColumnarData<'a> {

    fn new(ast: &'a syn::MacroInput, variant_data: &'a syn::VariantData) -> Self {
        let fields = match *variant_data {
            syn::VariantData::Struct(ref fields) => fields,
            ref e => panic!("Unsupported content: {:?}", e),
        };
        let type_ref: Ident = Ident::from(format!("{}Ref", ast.ident));
        let type_ref_mut: Ident = Ident::from(format!("{}RefMut", ast.ident));
        let type_columnar: Ident = Ident::from(format!("{}Columnar", ast.ident));
        let type_iter: Ident = Ident::from(format!("{}ColumnarIterator", ast.ident));
        let type_iter_mut: Ident = Ident::from(format!("{}ColumnarIteratorMut", ast.ident));
        Self {
            ast,
            fields,
            type_ref,
            type_ref_mut,
            type_columnar,
            type_iter,
            type_iter_mut,
        }
    }

    pub fn columnar_struct(&self) -> quote::Tokens {
        self.new_columnar_struct_impl()
    }

    fn new_columnar_struct_impl(&self) -> quote::Tokens {
        let ref_tokens = self.build_ref_type();
        let ref_mut_tokens = self.build_ref_mut_type();
        let columnar_tokens = self.build_columnar_type();
        let columnar_iterator_tokens = self.build_columnar_iterator_type(&self.type_iter, "::std::slice::Iter");
        let columnar_iterator_mut_tokens = self.build_columnar_iterator_type(&self.type_iter_mut, "::std::slice::IterMut");

        let columnar_impl = self.build_columnar_impl();
        let extend_impl = self.build_extend_impl();
        let into_iter_impl = self.build_into_iter_impl("iter", false);
        let into_iter_mut_impl = self.build_into_iter_impl("iter_mut", true);
        let ref_impl = self.build_ref_impl(&self.type_ref);
        let ref_mut_impl = self.build_ref_impl(&self.type_ref_mut);
        let columnar_iter_impl = self.build_columnar_iter_impl_iter(false);
        let columnar_iter_mut_impl = self.build_columnar_iter_impl_iter(true);
        let columnar_trait_impl = self.build_columnar_trait_impl();
        quote! {
            #ref_tokens

            #ref_mut_tokens

            #columnar_tokens

            #columnar_iterator_tokens

            #columnar_iterator_mut_tokens

            #columnar_impl

            #extend_impl

            #into_iter_impl

            #into_iter_mut_impl

            #ref_impl

            #ref_mut_impl

            #columnar_iter_impl

            #columnar_iter_mut_impl

            #columnar_trait_impl
        }
    }

    fn build_ref_type(&self) -> quote::Tokens {
        let lifetime_a = || syn::Lifetime { ident: Ident::from("'a") };
        let ref name = self.type_ref;

        let mut ref_type_generics: syn::Generics = self.ast.generics.clone();
        // Add 'a lifetime to Ref type
        ref_type_generics.lifetimes.push(syn::LifetimeDef::new("'a"));

        // Add same lifetime to the field refs
        let ref_type_fields: Vec<_> = self.fields.iter().map(|f| {
            let mut f = f.clone();
            f.ty = syn::Ty::Rptr(Some(lifetime_a()), Box::new(syn::MutTy { ty: f.ty, mutability: syn::Mutability::Immutable}));
            f
        }).collect();
        let (impl_generics, ty_generics, where_clause) = ref_type_generics.split_for_impl();
        quote! {
            #[derive(Debug)]
            #[allow(dead_code)]
            pub struct #name #ref_type_generics #where_clause {
                #(#ref_type_fields),*
            }
        }
    }

    fn build_ref_mut_type(&self) -> quote::Tokens {
        let lifetime_a = || syn::Lifetime { ident: Ident::from("'a") };
        let ref name = self.type_ref_mut;

        let mut ref_type_generics: syn::Generics = self.ast.generics.clone();
        // Add 'a lifetime to RefMut type
        ref_type_generics.lifetimes.push(syn::LifetimeDef::new("'a"));

        // Add same lifetime and mutability to the field refs
        let ref_type_fields: Vec<_> = self.fields.iter().map(|f| {
            let mut f = f.clone();
            f.ty = syn::Ty::Rptr(Some(lifetime_a()), Box::new(syn::MutTy { ty: f.ty, mutability: syn::Mutability::Mutable}));
            f
        }).collect();
        let (impl_generics, ty_generics, where_clause) = ref_type_generics.split_for_impl();
        quote! {
            #[allow(dead_code)]
            pub struct #name #ref_type_generics #where_clause {
                #(#ref_type_fields),*
            }
        }
    }

    fn build_columnar_type(&self) -> quote::Tokens {
        let ref name = self.type_columnar;

        // Encapsulate fields in Vec
        let ref_type_fields: Vec<_> = self.fields.iter().map(|f| {
            let mut f = f.clone();
            let mut segment = syn::PathSegment::from(syn::Ident::new("Vec"));
            let parameter_data = syn::AngleBracketedParameterData {
                lifetimes: vec![],
                types: vec![f.ty],
                bindings: vec![],
            };
            segment.parameters = syn::PathParameters::AngleBracketed(parameter_data);
            f.ty = syn::Ty::Path(None, syn::Path::from(segment));
            // f.ty = syn::Ty::Rptr(Some(lifetime_a()), Box::new(syn::MutTy { ty: f.ty, mutability: syn::Mutability::Mutable}));
            f
        }).collect();
        let (impl_generics, ty_generics, where_clause) = self.ast.generics.split_for_impl();
        quote! {
            #[allow(dead_code)]
            pub struct #name #ty_generics #where_clause {
                #(#ref_type_fields),*
            }
        }
    }
    fn build_columnar_iterator_type(&self, name: &Ident, iter_type_name: &str) -> quote::Tokens {
        let lifetime_a = || syn::Lifetime { ident: Ident::from("'a") };

        let mut ref_type_generics: syn::Generics = self.ast.generics.clone();
        // Add 'a lifetime to RefMut type
        ref_type_generics.lifetimes.push(syn::LifetimeDef::new("'a"));

        // Encapsulate fields in Vec
        let ref_type_fields: Vec<_> = self.fields.iter().map(|f| {
            let mut f = f.clone();
            let mut segment = syn::PathSegment::from(syn::Ident::new(iter_type_name));
            let parameter_data = syn::AngleBracketedParameterData {
                lifetimes: vec![lifetime_a()],
                types: vec![f.ty],
                bindings: vec![],
            };
            segment.parameters = syn::PathParameters::AngleBracketed(parameter_data);
            f.ty = syn::Ty::Path(None, syn::Path::from(segment));
            if let Some(ident) = f.ident {
                f.ident = Some(Ident::from(format!("iter_{}", ident)));
            }
            // f.ty = syn::Ty::Rptr(Some(lifetime_a()), Box::new(syn::MutTy { ty: f.ty, mutability: syn::Mutability::Mutable}));
            f
        }).collect();
        let (impl_generics, ty_generics, where_clause) = ref_type_generics.split_for_impl();
        quote! {
            #[allow(dead_code)]
            pub struct #name #ty_generics #where_clause {
                #(#ref_type_fields),*
            }
        }
    }

    fn build_columnar_impl(&self) -> quote::Tokens {
        let ref name = self.type_columnar;

        let new = self.build_columnar_new_impl();
        let with_capacity = self.build_columnar_with_capacity_impl();
        let iter = self.build_columnar_iter_impl(&self.type_iter, "iter", "");
        let iter_mut = self.build_columnar_iter_impl(&self.type_iter_mut, "iter_mut", "mut");

        quote! {
            impl #name {
                #new

                #with_capacity

                #iter

                #iter_mut
            }
        }
    }

    fn build_columnar_new_impl(&self) -> quote::Tokens {
        // Encapsulate fields in Vec
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        let ref name = self.type_columnar;

        quote! {
            pub fn new() -> Self {
                #name {
                    #(#names: Vec::new()),*
                }
            }
        }
    }

    fn build_columnar_with_capacity_impl(&self) -> quote::Tokens {
        // Encapsulate fields in Vec
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        let ref name = self.type_columnar;

        quote! {
            pub fn with_capacity(capacity: usize) -> Self {
                #name {
                    #(#names: Vec::with_capacity(capacity)),*
                }
            }
        }
    }

    fn build_columnar_iter_impl(&self, type_name: &Ident, iter: &str, modifier: &str) -> quote::Tokens {
        // Encapsulate fields in Vec
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        let iters: Vec<_> = self.fields.iter().map(|f| Ident::new(format!("iter_{}", f.ident.clone().unwrap()))).collect();
        let ref name = self.type_iter;
        let iter = Ident::new(iter);
        let fn_name = iter.clone();
        let iter = ::std::iter::repeat(iter);
        let modifier = Ident::new(modifier);
        quote! {
            pub fn #fn_name(& #modifier self) -> #type_name {
                #type_name {
                    #(#iters: self.#names.#iter()),*
                }
            }
        }
    }

    fn build_extend_impl(&self) -> quote::Tokens {
        // Encapsulate fields in Vec
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        let names2: Vec<_> = names.clone();
        let ref name = self.ast.ident;
        let ref type_columnar = self.type_columnar;

        quote! {
            impl Extend<#name> for #type_columnar {
                fn extend<T: IntoIterator<Item=#name>>(&mut self, iter: T) {
                    for element in iter {
                        #(self.#names.push(element.#names2));*
                    }
                }
            }
        }
    }

    fn build_into_iter_impl(&self, iter: &str, mutable: bool) -> quote::Tokens {
        // Encapsulate fields in Vec
        let ref type_columnar = self.type_columnar;

        let (mut_modifier, item, iter, call) = if mutable {
            (Ident::new("mut"), &self.type_ref_mut, &self.type_iter_mut, Ident::new("iter_mut"))
        } else {
            (Ident::new(""), &self.type_ref, &self.type_iter, Ident::new("iter"))
        };

        quote! {
            impl<'a> IntoIterator for &'a #mut_modifier #type_columnar {
                type Item = #item<'a>;
                type IntoIter = #iter<'a>;
                fn into_iter(self) -> Self::IntoIter {
                    self.#call()
                }
            }
        }
    }

    fn build_ref_impl(&self, type_ref: &Ident) -> quote::Tokens {
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        let names2: Vec<_> = names.clone();
        let ref name = self.ast.ident;

        quote! {
            #[allow(dead_code)]
            impl<'a> #type_ref<'a> {
                fn to_owned(&self) -> #name {
                    #name {
                        #(#names: *self.#names2),*
                    }
                }
            }
        }
    }

    fn build_columnar_iter_impl_iter(&self, mutable: bool) -> quote::Tokens {
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        // This is...ugly
        let names2 = names.clone();
        let names3 = names.clone();
        let names4 = names.clone();
        let names5 = names.clone();
        let iters: Vec<_> = self.fields.iter().map(|f| Ident::new(format!("iter_{}", f.ident.clone().unwrap()))).collect();
        let (type_iter, type_ref, modifier) = if mutable {
            (&self.type_iter_mut, &self.type_ref_mut, Ident::new("mut"))
        } else {
            (&self.type_iter, &self.type_ref, Ident::new(""))
        };

        let modifier_iter = ::std::iter::repeat(modifier.clone());

        quote! {
            impl<'a> Iterator for #type_iter<'a> {
                type Item = #type_ref<'a>;

                fn next<'b>(&'b mut self) -> Option<Self::Item> {
                    #(
                        let #names = self.#iters.next()
                    );*;
                    #(
                        if #names2.is_none() {
                            return None;
                        }
                    );*
                    #(
                        let #modifier_iter #names4 = #names5.unwrap()
                    );*;
                    Some(Self::Item {
                        #(#names3),*
                    })
                }
            }
        }
    }

    fn build_columnar_trait_impl(&self) -> quote::Tokens {
        let ref type_name = self.ast.ident;
        let ref type_ref_name = self.type_ref;
        let ref type_ref_mut_name = self.type_ref_mut;
        let ref type_columnar = self.type_columnar;
        let ref type_iter = self.type_iter;
        let ref type_iter_mut = self.type_iter_mut;
        quote! {
            impl<'a> ::columnar::Columnar<'a> for #type_name {
                type Ref = #type_ref_name<'a>;
                type RefMut = #type_ref_name<'a>;
                type Columnar = #type_columnar;
                type Iter = #type_iter<'a>;
                type IterMut = #type_iter_mut<'a>;
            }
        }
    }

}
