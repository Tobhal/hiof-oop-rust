use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Ident, Type};
use syn::__private::TokenStream2;

// List of all types that will be filtered
const SUPPORTED_TYPES: [&'static str; 17] = [
    "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64",
    "bool", "char", "String"
];

fn filter_values(f: Field) -> bool {
    match f.ty {
        syn::Type::Path(path) => SUPPORTED_TYPES.contains(&path.path.segments[0].ident.to_string().as_str()),
        _ => false
    }
}

fn type_to_string(ident: Ident, t: Type) -> TokenStream2 {
    match t {
        syn::Type::Path(path) => if SUPPORTED_TYPES.contains(&path.path.segments[0].ident.to_string().as_str()) {
            quote! { self.#ident.to_string() }
        } else {
            quote! { "Vec".to_string() }
        },
        _ => quote! { "unknown".to_string() }
    }
}


fn impl_field_editable_trait(ast: DeriveInput) -> TokenStream {
    // get struct info
    let ident = ast.ident;

    // get field idents
    let (field_idents, field_types): (Vec<Ident>, Vec<Type>) = match ast.data {
        syn::Data::Struct(data) => (
            data.fields.clone().into_iter().filter(|f| filter_values(f.clone())).filter_map(|f| f.ident).collect(),
            data.fields.into_iter().filter(|f| filter_values(f.clone())).map(|f| f.ty).collect()
        ),
        syn::Data::Union(_) => panic!("field editable not supported for union"),
        syn::Data::Enum(_) => panic!("field editable not supported for enum")
    };

    let field_ident_strs: Vec<String> = field_idents.iter().map(|i| i.to_string()).collect();

    let val_string: Vec<TokenStream2> = field_types.iter().enumerate().map(|(i, t)| type_to_string(field_idents[i].clone(), t.clone())).collect();

    // generate impl
    let quote = quote! {
        impl FieldEditable for #ident {
            fn get_fields(&self) -> Vec<(&'static str, String)> {
                vec![#((#field_ident_strs, #val_string)),*]
            }

            fn edit_field(&mut self, field: &'static str, value: String) -> Result<(), Box<dyn Error>> {
                let field_string = field.to_string();
                let value_string = value.to_string();

                #(
                    if field_string.eq(#field_ident_strs) {
                        self.#field_idents = match value_string.parse() {
                            Ok(val) => val,
                            Err(err) => return Err(Box::new(err))
                        }
                    }
                )*

                Ok(())
            }
        }
    };

    quote.into()
}

#[proc_macro_derive(FieldEditable)]
pub fn field_editable_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_field_editable_trait(ast)
}