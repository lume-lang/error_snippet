use syn::{Error, Field, Result};

use crate::args::DiagnosticArg;

impl DiagnosticArg {
    pub fn parse_field(field: &Field) -> Result<Option<Self>> {
        let attr = match field.attrs.first() {
            Some(attr) => attr,
            None => return Ok(None),
        };

        let attr_path = attr.path();

        let field_ident = field.ident.as_ref().unwrap();
        let attr_ident = attr_path.get_ident().unwrap();

        let arg = match attr_ident.to_string().as_str() {
            "span" => {
                if let syn::Meta::Path(_) = &attr.meta {
                    DiagnosticArg::Span(field_ident.clone())
                } else {
                    return Err(Error::new_spanned(
                        attr_path,
                        "expected no arguments; should be formatted `#[span]`",
                    ));
                }
            }
            "related" => {
                if let syn::Meta::Path(_) = &attr.meta {
                    DiagnosticArg::Related(field_ident.clone())
                } else {
                    return Err(Error::new_spanned(
                        attr_path,
                        "expected no arguments; should be formatted `#[related]`",
                    ));
                }
            }
            "label" => {
                if let syn::Meta::List(meta) = &attr.meta {
                    Self::parse_label(field_ident, meta)?
                } else {
                    return Err(Error::new_spanned(
                        attr_path,
                        "expected list argument; should be formatted `#[label(\"...\")]`",
                    ));
                }
            }
            unk => {
                return Err(Error::new_spanned(
                    attr_path,
                    format!("unknown property attribute: {}", unk),
                ))
            }
        };

        Ok(Some(arg))
    }

    fn parse_label(ident: &syn::Ident, list: &syn::MetaList) -> Result<Self> {
        if let Ok(label) = list.parse_args::<syn::LitStr>() {
            Ok(DiagnosticArg::Label(label.value(), ident.clone()))
        } else {
            Err(Error::new_spanned(list, "expected attribute argument"))
        }
    }
}
