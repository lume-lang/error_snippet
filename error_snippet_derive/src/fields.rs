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
            "related" => match &attr.meta {
                syn::Meta::Path(_) => DiagnosticArg::Related(field_ident.clone(), false),
                syn::Meta::List(meta) => Self::parse_related(field_ident, meta)?,
                _ => {
                    return Err(Error::new_spanned(
                        attr_path,
                        "expected zero-or-one arguments; should be formatted `#[related]` or `#[related(collection)]`",
                    ))
                }
            }
            "cause" => {
                if let syn::Meta::Path(_) = &attr.meta {
                    DiagnosticArg::Cause(field_ident.clone(), false)
                } else {
                    return Err(Error::new_spanned(
                        attr_path,
                        "expected no arguments; should be formatted `#[cause]`",
                    ));
                }
            }
            "causes" => {
                if let syn::Meta::Path(_) = &attr.meta {
                    DiagnosticArg::Cause(field_ident.clone(), true)
                } else {
                    return Err(Error::new_spanned(
                        attr_path,
                        "expected no arguments; should be formatted `#[causes]`",
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
                    format!("unknown property attribute: {unk}"),
                ))
            }
        };

        Ok(Some(arg))
    }

    fn parse_related(ident: &syn::Ident, list: &syn::MetaList) -> Result<Self> {
        let parser = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated;

        let mut collection = false;

        for arg in list.parse_args_with(parser)? {
            match arg.get_ident().unwrap().to_string().as_str() {
                "collection" => collection = true,
                value => return Err(Error::new_spanned(arg, format!("invalid option, {value}"))),
            }
        }

        Ok(DiagnosticArg::Related(ident.clone(), collection))
    }

    fn parse_label(ident: &syn::Ident, list: &syn::MetaList) -> Result<Self> {
        let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;

        let mut has_source = false;
        let mut label_str = None;

        for label_arg in list.parse_args_with(parser)? {
            match label_arg {
                syn::Expr::Path(syn::ExprPath { path, .. }) => {
                    match path.get_ident().unwrap().to_string().as_str() {
                        "source" => has_source = true,
                        value => {
                            return Err(Error::new_spanned(
                                path,
                                format!("invalid option, {value}"),
                            ))
                        }
                    }
                }
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) => {
                    label_str = Some(lit_str.value());
                }
                _ => (),
            }
        }

        if let Some(label) = label_str {
            Ok(DiagnosticArg::Label(label, ident.clone(), has_source))
        } else {
            Err(Error::new_spanned(list, "expected attribute argument"))
        }
    }
}
