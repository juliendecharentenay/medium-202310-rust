
#[proc_macro_derive(DeriveDraw)]
pub fn draw(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  syn::parse(input)
  .and_then(|ast: syn::DeriveInput|
    match &ast.data {
      syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(named), .. }) => implement_draw(&ast, named),
      _ => Err(syn::parse::Error::new(ast.ident.span(), "Apply #[derive(DeriveDraw)] to struct with named fields")),
    }
  )
  .unwrap_or_else(|e| e.into_compile_error())
  .into()
}

fn implement_draw(ast: &syn::DeriveInput, named_fields: &syn::FieldsNamed) -> syn::parse::Result<proc_macro2::TokenStream> {
  let ident = &ast.ident;
  let points = named_fields.named
      .iter()
      .filter(|field| field.ident.is_some())
      .filter(|field| 
        if let syn::Type::Path(t_path) = &field.ty {
          if let Some(path_segment) = t_path.path.segments.last() {
            path_segment.ident.eq("Point")
          } else {
            false
          }
        } else {
          false
        }
      )
      .map(|field| field.ident.as_ref().unwrap())
      .collect::<Vec<&syn::Ident>>();
  Ok(
    quote::quote! {
      impl #ident {
        fn draw(&self) -> String {
           let points = vec![ #( self.#points, )* ];
           points.iter()
           .fold("".to_string(),
             |r, p| if r.len() == 0 { format!("M {} {}", p.x, p.y) } else { format!("{} L {} {}", r, p.x, p.y) }
           ) + format!(" L {} {}", points[0].x, points[0].y).as_str()
        }
      }
    }
  )
}

