extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named, .. }), ..  }) = &ast.data {
        named
    } else {
      unimplemented!();
    };
    let exp = quote! {
        pub struct #bident {
         executable: Option<String>,
         args: Option<Vec<String>>,
         env: Option<Vec<String>>,
         current_dir: Option<String>,
        }

        impl #bident {
        pub fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = Some(executable);
        self
        }

                pub fn env(&mut self, env: Vec<String>) -> &mut Self {
        self.env = Some(env);
        self
        }

                        pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
        self.current_dir = Some(current_dir);
        self
        }

                        pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = Some(args);
        self
        }

        pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
        Ok(#name {
        executable: self.executable.clone().ok_or("executable is not set")?,
        env: self.env.clone().ok_or("env is not set")?,
        args: self.args.clone().ok_or("args is not set")?,
        current_dir: self.current_dir.clone().ok_or("current_dir is not set")?,
        })
        }
        }

        impl #name {
           fn builder() -> #bident {
           #bident {
                 executable: None,
                 args: None,
                 env: None,
                 current_dir: None,
             }
           }
        }
    };

    exp.into()
}
