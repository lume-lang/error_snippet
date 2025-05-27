use syn::{Attribute, Error, Ident, MetaNameValue, Result};

use crate::diagnostic::Severity;

#[derive(Debug)]
pub enum DiagnosticArg {
    Message(String),
    Code(String),
    Help(String),
    Severity(Severity),
    Related(Ident),
    Cause(Ident),
    Span(Ident),
    Label(String, Ident),
}

impl DiagnosticArg {
    pub fn parse_attributes(attributes: &[Attribute]) -> Result<Vec<Self>> {
        let mut args = Vec::new();

        for attribute in attributes {
            args.extend(Self::parse_attribute(attribute)?);
        }

        Ok(args)
    }

    pub fn parse_attribute(attr: &Attribute) -> Result<Vec<Self>> {
        if attr.path().is_ident("diagnostic") {
            DiagnosticArg::parse_diagnostic(attr)
        } else {
            Err(Error::new_spanned(attr, "unknown attribute"))
        }
    }

    fn parse_diagnostic(attr: &Attribute) -> Result<Vec<Self>> {
        if let syn::Meta::List(meta) = &attr.meta {
            let mut args = Vec::new();
            let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;

            for arg_meta in meta.parse_args_with(parser)? {
                if let syn::Meta::NameValue(name_value) = arg_meta {
                    let arg = Self::parse_diagnostic_argument(&name_value)?;

                    args.push(arg);
                } else {
                    return Err(Error::new_spanned(
                        meta,
                        "expected name-value attribute, such as `#[attr = \"...\"]`",
                    ));
                }
            }

            Ok(args)
        } else {
            Err(Error::new_spanned(attr, "Expected list attribute"))
        }
    }

    fn parse_diagnostic_argument(name_value: &MetaNameValue) -> Result<Self> {
        let ident = match name_value.path.get_ident() {
            Some(ident) => ident,
            None => return Err(Error::new_spanned(&name_value.path, "Expected identifier")),
        };

        match ident.to_string().as_str() {
            "code" => Self::parse_code(name_value),
            "message" => Self::parse_message(name_value),
            "help" => Self::parse_help(name_value),
            "severity" => Self::parse_severity(name_value),
            _ => Err(Error::new_spanned(ident, "Invalid diagnostic attribute")),
        }
    }

    fn parse_message(meta: &MetaNameValue) -> Result<Self> {
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(lit_str),
            ..
        }) = meta.value.clone()
        {
            Ok(DiagnosticArg::Message(lit_str.value()))
        } else {
            Err(Error::new_spanned(meta, "Expected string literal"))
        }
    }

    fn parse_code(meta: &MetaNameValue) -> Result<Self> {
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(lit_str),
            ..
        }) = meta.value.clone()
        {
            Ok(DiagnosticArg::Code(lit_str.value()))
        } else {
            Err(Error::new_spanned(meta, "Expected string literal"))
        }
    }

    fn parse_help(meta: &MetaNameValue) -> Result<Self> {
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(lit_str),
            ..
        }) = meta.value.clone()
        {
            Ok(DiagnosticArg::Help(lit_str.value()))
        } else {
            Err(Error::new_spanned(meta, "Expected string literal"))
        }
    }

    fn parse_severity(meta: &MetaNameValue) -> Result<Self> {
        if let syn::Expr::Path(syn::ExprPath { path, .. }) = meta.value.clone() {
            let ident = match path.get_ident() {
                Some(ident) => ident,
                None => return Err(Error::new_spanned(path, "Expected ident for path")),
            };

            Ok(DiagnosticArg::Severity(Severity(ident.clone())))
        } else {
            Err(Error::new_spanned(meta, "Expected path literal"))
        }
    }
}
