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



const COLUMNAR_LIFETIME: &str = "'columnar";

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
        match print_generated_code(result_string, &ast, source) {
            Err(reason) => panic!(reason),
            Ok(_) => {},
        }
    }
    result.to_string().parse().unwrap()
}

#[cfg(feature = "verbose")]
fn print_generated_code(result_string: String, ast: &syn::MacroInput, source: String) -> ::std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::Write;

    // Use rustfmt for pretty-printing
    let input = rustfmt::Input::Text(result_string);
    let config = rustfmt::config::Config::default();
    let (error_summary, file_map, _report) = rustfmt::format_input::<std::io::Stdout>(input, &config, None)
        .unwrap();
    assert!(error_summary.has_no_errors());
    for &(ref file_name, ref text) in &file_map {
        if file_name == "stdin" {
            let text = text.to_string();
            let mut file = File::create(format!("target/derive_columnar_{}.rs", ast.ident.as_ref())).expect("Failed to open file");
            file.write_all(format!("// AST: {:?}\n", ast).as_bytes())?;
            file.write_all(b"extern crate columnar;\nuse columnar::Columnar;\n")?;
            file.write_all(source.as_bytes())?;
            file.write_all(b"\n")?;
            file.write_all(text.as_bytes())?;
            break;
        }
    }
    Ok(())
}

#[cfg(not(feature = "verbose"))]
fn print_generated_code(_result_string: String, _ast: &syn::MacroInput, _source: String) -> ::std::io::Result<()> {
    Ok(())
}
struct ColumnarData<'a> {
    type_ref: Ident,
    type_ref_mut: Ident,
    type_container: Ident,
    type_iter: Ident,
    type_iter_mut: Ident,

    ast: &'a syn::MacroInput,
    fields: &'a [syn::Field],

    lt_generics: syn::Generics,
}

impl<'a> ColumnarData<'a> {

    fn new(ast: &'a syn::MacroInput, variant_data: &'a syn::VariantData) -> Self {
        let fields = match *variant_data {
            syn::VariantData::Struct(ref fields) => fields,
            syn::VariantData::Tuple(ref elements) => panic!("Unsupported content: {:?}", elements),
            syn::VariantData::Unit => panic!("Unsupported content: Unit"),
        };
        let type_ref: Ident = Ident::from(format!("{}Ref", ast.ident));
        let type_ref_mut: Ident = Ident::from(format!("{}RefMut", ast.ident));
        let type_container: Ident = Ident::from(format!("{}Columnar", ast.ident));
        let type_iter: Ident = Ident::from(format!("{}ColumnarIterator", ast.ident));
        let type_iter_mut: Ident = Ident::from(format!("{}ColumnarIteratorMut", ast.ident));

        let mut lt_generics = ast.generics.clone();
        lt_generics.lifetimes.push(syn::LifetimeDef::new(COLUMNAR_LIFETIME));


        // Add a where X: 'lifetime to every generic parameter
        for ty_param in &lt_generics.ty_params {
            let mut segment = syn::PathSegment::from(ty_param.ident.clone());
            let parameter_data = syn::AngleBracketedParameterData {
                lifetimes: vec![],
                types: vec![],
                bindings: vec![],
            };
            segment.parameters = syn::PathParameters::AngleBracketed(parameter_data);

            let where_bound = syn::WhereBoundPredicate {
                bound_lifetimes: vec![],
                bounded_ty: syn::Ty::Path(None, syn::Path::from(segment)),
                bounds: vec![syn::TyParamBound::Region(syn::Lifetime::new(COLUMNAR_LIFETIME))],
            };
            lt_generics.where_clause.predicates.push(syn::WherePredicate::BoundPredicate(where_bound));
        }

        Self {
            ast,
            fields,
            type_ref,
            type_ref_mut,
            type_container,
            type_iter,
            type_iter_mut,
            lt_generics,
        }
    }

    fn columnar_struct(&self) -> quote::Tokens {
        self.new_columnar_struct_impl()
    }

    fn new_columnar_struct_impl(&self) -> quote::Tokens {
        let ref_tokens = self.build_ref_type();
        let ref_mut_tokens = self.build_ref_mut_type();
        let columnar_tokens = self.build_columnar_type();
        let columnar_iterator_tokens = self.build_columnar_iterator_type(&self.type_iter, "::std::slice::Iter");
        let columnar_iterator_mut_tokens = self.build_columnar_iterator_type(&self.type_iter_mut, "::std::slice::IterMut");

        let container_impl = self.build_container_impl();
        let extend_impl = self.build_extend_impl();
        let into_iter_impl = self.build_into_iter_impl(false);
        let into_iter_mut_impl = self.build_into_iter_impl(true);
        let ref_impl = self.build_ref_impl(&self.type_ref);
        let ref_mut_impl = self.build_ref_impl(&self.type_ref_mut);
        let columnar_iter_impl = self.build_columnar_iter_impl_iter(false);
        let columnar_iter_mut_impl = self.build_columnar_iter_impl_iter(true);
        quote! {

            #ref_tokens

            #ref_mut_tokens

            #columnar_tokens

            #columnar_iterator_tokens

            #columnar_iterator_mut_tokens

            #container_impl

            #extend_impl

            #into_iter_impl

            #into_iter_mut_impl

            #ref_impl

            #ref_mut_impl

            #columnar_iter_impl

            #columnar_iter_mut_impl
        }
    }

    fn build_ref_type(&self) -> quote::Tokens {
        let lifetime_a = || syn::Lifetime { ident: Ident::from(COLUMNAR_LIFETIME) };
        let ref name = self.type_ref;

        // Add same lifetime to the field refs
        let ref_type_fields: Vec<_> = self.fields.iter().map(|f| {
            let mut f = f.clone();
            f.ty = syn::Ty::Rptr(Some(lifetime_a()), Box::new(syn::MutTy { ty: f.ty, mutability: syn::Mutability::Immutable}));
            f
        }).collect();
        let (_impl_generics, ty_generics, where_clause) = self.lt_generics.split_for_impl();
        let ref vis = self.ast.vis;
        quote! {
            #[derive(Debug)]
            #[allow(dead_code)]
            #vis struct #name #ty_generics #where_clause {
                #(#ref_type_fields),*
            }
        }
    }

    fn build_ref_mut_type(&self) -> quote::Tokens {
        let lifetime_a = || syn::Lifetime { ident: Ident::from(COLUMNAR_LIFETIME) };
        let ref name = self.type_ref_mut;

        // Add same lifetime and mutability to the field refs
        let ref_type_fields: Vec<_> = self.fields.iter().map(|f| {
            let mut f = f.clone();
            f.ty = syn::Ty::Rptr(Some(lifetime_a()), Box::new(syn::MutTy { ty: f.ty, mutability: syn::Mutability::Mutable}));
            f
        }).collect();
        let (_impl_generics, ty_generics, where_clause) = self.lt_generics.split_for_impl();
        let ref vis = self.ast.vis;
        quote! {
            #[derive(Debug)]
            #[allow(dead_code)]
            #vis struct #name #ty_generics #where_clause {
                #(#ref_type_fields),*
            }
        }
    }

    fn build_columnar_type(&self) -> quote::Tokens {
        let ref name = self.type_container;

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
            f
        }).collect();
        let (_impl_generics, ty_generics, where_clause) = self.ast.generics.split_for_impl();
        let ref vis = self.ast.vis;
        quote! {
            #[derive(Debug)]
            #[allow(dead_code)]
            #vis struct #name #ty_generics #where_clause {
                #(#ref_type_fields),*
            }
        }
    }
    fn build_columnar_iterator_type(&self, name: &Ident, iter_type_name: &str) -> quote::Tokens {
        let lifetime_a = || syn::Lifetime { ident: Ident::from(COLUMNAR_LIFETIME) };

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
            f
        }).collect();
        let (_impl_generics, ty_generics, where_clause) = self.lt_generics.split_for_impl();
        let ref vis = self.ast.vis;
        quote! {
            #[derive(Debug)]
            #[allow(dead_code)]
            #vis struct #name #ty_generics #where_clause {
                #(#ref_type_fields),*
            }
        }
    }

    fn build_container_impl(&self) -> quote::Tokens {
        let ref type_continer = self.type_container;
        let ref type_columnar = self.ast.ident;

        let (_impl_generics, ty_generics, _where_clause) = self.ast.generics.split_for_impl();
        let (lt_impl_generics, lt_ty_generics, lt_where_clause) = self.lt_generics.split_for_impl();
        let lifetime = syn::Lifetime { ident: Ident::from(COLUMNAR_LIFETIME) };

        let new = self.build_columnar_new_impl();
        let with_capacity = self.build_columnar_with_capacity_impl();
        let iter = self.build_columnar_iter_impl(&self.type_iter, "iter", "", &ty_generics);
        let iter_mut = self.build_columnar_iter_impl(&self.type_iter_mut, "iter_mut", "mut", &ty_generics);
        let len = self.build_columnar_len_impl();

        let ref type_ref_name = self.type_ref;
        let ref type_ref_mut_name = self.type_ref_mut;
        let ref type_container = self.type_container;
        let ref type_iter = self.type_iter;
        let ref type_iter_mut = self.type_iter_mut;

        quote! {
            impl#lt_impl_generics ::columnar::Columnar<#lifetime> for #type_continer #ty_generics #lt_where_clause {

                type Ref = #type_ref_name #lt_ty_generics;
                type RefMut = #type_ref_mut_name #lt_ty_generics;
                type Iter = #type_iter #lt_ty_generics;
                type IterMut = #type_iter_mut #lt_ty_generics;

                #iter
                #iter_mut
                #len
            }

            #[allow(dead_code)]
            impl#lt_impl_generics ::columnar::ColumnarFactory<#lifetime> for #type_columnar #ty_generics #lt_where_clause {
                type Output = #type_container #ty_generics;

                #new

                #with_capacity

            }
        }
    }

    fn build_columnar_new_impl(&self) -> quote::Tokens {
        // Encapsulate fields in Vec
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        let ref name = self.type_container;

        quote! {
            fn new() -> Self::Output {
                #name {
                    #(#names: Vec::new()),*
                }
            }
        }
    }

    fn build_columnar_with_capacity_impl(&self) -> quote::Tokens {
        // Encapsulate fields in Vec
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        let ref name = self.type_container;

        quote! {
            fn with_capacity(capacity: usize) -> Self::Output {
                #name {
                    #(#names: Vec::with_capacity(capacity)),*
                }
            }
        }
    }

    fn build_columnar_iter_impl(&self, type_name: &Ident, iter: &str, modifier: &str, ty_generics: &syn::TyGenerics) -> quote::Tokens {
        // Encapsulate fields in Vec
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        let iters: Vec<_> = self.fields.iter().map(|f| Ident::new(format!("iter_{}", f.ident.clone().unwrap()))).collect();
        let iter = Ident::new(iter);
        let fn_name = iter.clone();
        let iter = ::std::iter::repeat(iter);
        let modifier = Ident::new(modifier);
        quote! {
            fn #fn_name(& #modifier self) -> #type_name #ty_generics {
                #type_name {
                    #(#iters: self.#names.#iter()),*
                }
            }
        }
    }

    fn build_columnar_len_impl(&self) -> quote::Tokens {
        let name = if let Some(name) = self.fields.first().expect("At least one field required").clone().ident {
            name
        } else {
            syn::Ident::new("0")
        };
        quote! {
            fn len(&self) -> usize {
                self.#name.len()
            }
        }
    }

    fn build_extend_impl(&self) -> quote::Tokens {
        // Encapsulate fields in Vec
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        let names2: Vec<_> = names.clone();
        let ref name = self.ast.ident;
        let ref type_container = self.type_container;
        let (_impl_generics, ty_generics, _where_clause) = self.ast.generics.split_for_impl();

        let (lt_impl_generics, _lt_ty_generics, lt_where_clause) = self.lt_generics.split_for_impl();

        quote! {
            impl #lt_impl_generics Extend<#name#ty_generics> for #type_container #ty_generics #lt_where_clause {
                fn extend<T: IntoIterator<Item=#name#ty_generics>>(&mut self, iter: T) {
                    for element in iter {
                        #(self.#names.push(element.#names2));*
                    }
                }
            }
        }
    }

    fn build_into_iter_impl(&self, mutable: bool) -> quote::Tokens {
        // Encapsulate fields in Vec
        let ref type_container = self.type_container;

        let (lt_impl_generics, lt_ty_generics, _lt_where_clause) = self.lt_generics.split_for_impl();
        let (_impl_generics, ty_generics, where_clause) = self.ast.generics.split_for_impl();


        let (mut_modifier, item, iter, call) = if mutable {
            (Ident::new("mut"), &self.type_ref_mut, &self.type_iter_mut, Ident::new("iter_mut"))
        } else {
            (Ident::new(""), &self.type_ref, &self.type_iter, Ident::new("iter"))
        };
        let lifetime = Ident::from(COLUMNAR_LIFETIME);
        quote! {
            impl#lt_impl_generics IntoIterator for &#lifetime #mut_modifier #type_container #ty_generics #where_clause{
                type Item = #item#lt_ty_generics;
                type IntoIter = #iter#lt_ty_generics;
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

        let (lt_impl_generics, lt_ty_generics, lt_where_clause) = self.lt_generics.split_for_impl();
        let (_impl_generics, ty_generics, _where_clause) = self.ast.generics.split_for_impl();

        quote! {
            #[allow(dead_code)]
            impl #lt_impl_generics #type_ref #lt_ty_generics #lt_where_clause {
                fn to_owned(&self) -> #name#ty_generics {
                    #name {
                        #(#names: *self.#names2),*
                    }
                }
            }
        }
    }

    fn build_columnar_iter_impl_iter(&self, mutable: bool) -> quote::Tokens {
        let names: Vec<_> = self.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
        // This is...ugly. quote! seems to consume the thing and Ident doesn't implement Copy.
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

        let (impl_generics, ty_generics, where_clause) = self.lt_generics.split_for_impl();

        let modifier_iter = ::std::iter::repeat(modifier.clone());

        quote! {
            impl #impl_generics Iterator for #type_iter #ty_generics #where_clause {
                type Item = #type_ref #ty_generics;

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

}
