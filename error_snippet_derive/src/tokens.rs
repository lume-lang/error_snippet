use crate::{
    args::DiagnosticArg,
    diagnostic::{Diagnostic, Severity},
    fmt::FormattedMessage,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

impl Diagnostic {
    pub fn tokens(&self) -> syn::Result<TokenStream> {
        let (impl_gen, ty_gen, where_clause) = &self.generics.split_for_impl();

        let name = &self.ident;
        let message_block = self.message_block()?;
        let code_block = self.code_block()?;
        let help_block = self.help_block()?;
        let labels_block = self.labels_block()?;
        let related_block = self.related_block()?;
        let cause_block = self.cause_block()?;
        let source_block = self.source_block()?;
        let severity_block = self.severity_block()?;

        let stream = quote! {
            impl #impl_gen ::error_snippet::Diagnostic for #name #ty_gen #where_clause {
                #message_block
                #code_block
                #help_block
                #labels_block
                #related_block
                #cause_block
                #source_block
                #severity_block
            }

            impl ::std::error::Error for #name {}

            impl ::std::fmt::Display for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}", ::error_snippet::Diagnostic::message(self))
                }
            }
        };

        Ok(stream)
    }

    /// Gets the value of the `message` attribute, if any was given. If not, raises
    /// an error for the user.
    pub fn message(&self) -> syn::Result<String> {
        let arg = self
            .args
            .iter()
            .find(|arg| matches!(arg, DiagnosticArg::Message(_)));

        match arg {
            Some(DiagnosticArg::Message(message)) => Ok(message.clone()),
            _ => Err(self
                .err("No error message provided. Please use `#[diagnostic(message = \"...\")]`")),
        }
    }

    /// Gets the value of the `code` attribute, if any was given. If not, returns `None`.
    pub fn code(&self) -> Option<String> {
        let arg = self
            .args
            .iter()
            .find(|arg| matches!(arg, DiagnosticArg::Code(_)));

        match arg {
            Some(DiagnosticArg::Code(code)) => Some(code.clone()),
            _ => None,
        }
    }

    /// Gets the value(s) of the `help` attribute(s), if any was given. If not, returns `None`.
    pub fn help(&self) -> Option<Vec<String>> {
        let args = self
            .args
            .iter()
            .filter_map(|arg| {
                if let DiagnosticArg::Help(help) = arg {
                    Some(help.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        if args.is_empty() {
            None
        } else {
            Some(args)
        }
    }

    /// Gets the source code of the diagnostic, if any was given. If not, returns `None`.
    pub fn span(&self) -> Option<Ident> {
        let arg = self
            .args
            .iter()
            .find(|arg| matches!(arg, DiagnosticArg::Span(_)));

        match arg {
            Some(DiagnosticArg::Span(span)) => Some(span.clone()),
            _ => None,
        }
    }

    /// Gets the severity of the diagnostic, if any was given. If not, returns `None`.
    pub fn severity(&self) -> Option<Severity> {
        let arg = self
            .args
            .iter()
            .find(|arg| matches!(arg, DiagnosticArg::Severity(_)));

        match arg {
            Some(DiagnosticArg::Severity(severity)) => Some(severity.clone()),
            _ => None,
        }
    }

    /// Gets the value(s) of the `labels` attribute(s), if any was given. If not, returns `None`.
    pub fn labels(&self) -> Option<Vec<(String, Ident)>> {
        let args = self
            .args
            .iter()
            .filter_map(|arg| {
                if let DiagnosticArg::Label(label, ident) = arg {
                    Some((label.clone(), ident.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, Ident)>>();

        if args.is_empty() {
            None
        } else {
            Some(args)
        }
    }

    /// Creates the implementation block for the `message` trait function.
    fn message_block(&self) -> syn::Result<TokenStream> {
        let message = self.message()?;
        let lit = syn::LitStr::new(&message, proc_macro2::Span::call_site());

        let formatted = FormattedMessage::expand(lit);

        let stream = quote! {
            fn message<'a>(&'a self) -> String {
                #formatted
            }
        };

        Ok(stream)
    }

    /// Creates the implementation block for the `code` trait function.
    fn code_block(&self) -> syn::Result<TokenStream> {
        let stream = if let Some(code) = self.code() {
            quote! {
                fn code(&self) -> Option<Box<dyn std::fmt::Display + '_>> {
                    Some(Box::new(#code) as Box<dyn std::fmt::Display + '_>)
                }
            }
        } else {
            TokenStream::new()
        };

        Ok(stream)
    }

    /// Creates the implementation block for the `help` trait function.
    fn help_block(&self) -> syn::Result<TokenStream> {
        let stream = if let Some(help) = self.help() {
            let help_idents = help
                .into_iter()
                .map(|h| {
                    let lit = syn::LitStr::new(&h, proc_macro2::Span::call_site());

                    FormattedMessage::expand(lit)
                })
                .collect::<Vec<TokenStream>>();

            quote! {
                fn help(&self) -> Option<Box<dyn Iterator<Item = ::error_snippet::Help> + '_>> {
                    Some(Box::new(
                        vec![ #(#help_idents),* ]
                            .into_iter()
                            .map(|h| Into::<::error_snippet::Help>::into(h))
                    ))
                }
            }
        } else {
            TokenStream::new()
        };

        Ok(stream)
    }

    /// Creates the implementation block for the `labels` trait function.
    fn labels_block(&self) -> syn::Result<TokenStream> {
        let stream = if let Some(labels) = self.labels() {
            let label_pairs = labels
                .into_iter()
                .map(|(label, ident)| {
                    let lit_str = syn::LitStr::new(&label, proc_macro2::Span::call_site());
                    let formatted_str = FormattedMessage::expand(lit_str);

                    quote! {
                        ::error_snippet::Label::new(
                            ::error_snippet::Diagnostic::source_code(self),
                            self.#ident.clone(),
                            #formatted_str
                        )
                    }
                })
                .collect::<Vec<TokenStream>>();

            quote! {
                fn labels(&self) -> Option<Box<dyn Iterator<Item = ::error_snippet::Label> + '_>> {
                    let labels = Box::new(vec![ #(#label_pairs),* ].into_iter());

                    Some(labels)
                }
            }
        } else {
            TokenStream::new()
        };

        Ok(stream)
    }

    /// Creates the implementation block for the `related` trait function.
    fn related_block(&self) -> syn::Result<TokenStream> {
        let arg = self
            .args
            .iter()
            .find(|arg| matches!(arg, DiagnosticArg::Related(_)));
        let related = match arg {
            Some(DiagnosticArg::Related(related)) => related.clone(),
            _ => return Ok(TokenStream::new()),
        };

        let stream = quote! {
            fn related(&self) -> Box<dyn Iterator<Item = &(dyn ::error_snippet::Diagnostic + Send + Sync)> + '_> {
                Box::new(
                    self.#related
                        .iter()
                        .map(|e| e.as_ref() as &(dyn ::error_snippet::Diagnostic + Send + Sync)),
                )
            }
        };

        Ok(stream)
    }

    /// Creates the implementation block for the `cause` trait function.
    fn cause_block(&self) -> syn::Result<TokenStream> {
        let arg = self
            .args
            .iter()
            .find(|arg| matches!(arg, DiagnosticArg::Cause(_)));
        let cause = match arg {
            Some(DiagnosticArg::Cause(cause)) => cause.clone(),
            _ => return Ok(TokenStream::new()),
        };

        let stream = quote! {
            fn causes(&self) -> Box<dyn Iterator<Item = &(dyn ::error_snippet::Diagnostic + Send + Sync)> + '_> {
                Box::new(
                    self.#cause
                        .iter()
                        .map(|e| e.as_ref() as &(dyn ::error_snippet::Diagnostic + Send + Sync)),
                )
            }
        };

        Ok(stream)
    }

    /// Creates the implementation block for the `source_code` trait function.
    fn source_block(&self) -> syn::Result<TokenStream> {
        let stream = if let Some(span) = self.span() {
            quote! {
                fn source_code(&self) -> Option<std::sync::Arc<dyn ::error_snippet::Source>> {
                    Some(self.#span.clone())
                }
            }
        } else {
            TokenStream::new()
        };

        Ok(stream)
    }

    /// Creates the implementation block for the `severity` trait function.
    fn severity_block(&self) -> syn::Result<TokenStream> {
        let stream = if let Some(severity) = self.severity() {
            let path = &severity.0;

            quote! {
                fn severity(&self) -> ::error_snippet::Severity {
                    ::error_snippet::Severity::#path
                }
            }
        } else {
            TokenStream::new()
        };

        Ok(stream)
    }
}
