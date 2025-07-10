use proc_macro2::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, parse::Parser, Ident, LitStr};

/// Determines whether the given substring is a debug print format.
fn is_debug_print(read: &str) -> bool {
    let Some(brace) = read.find('}') else {
        return false;
    };

    let Some(debug) = read.find('?') else {
        return false;
    };

    debug < brace
}

pub struct FormattedMessage {
    format: LitStr,
}

impl FormattedMessage {
    pub fn expand(str: LitStr) -> TokenStream {
        let message = FormattedMessage { format: str };

        message.expand_format()
    }

    pub fn expand_format(&self) -> TokenStream {
        let span = self.format.span();
        let fmt = self.format.value();
        let mut args = Vec::new();

        let mut read = fmt.as_str();

        while let Some(brace) = read.find('{') {
            read = &read[brace + 1..];

            let next = match read.chars().next() {
                Some(c) => c,
                None => break,
            };

            let ident = match next {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = Self::read_ident(&mut read);
                    ident.set_span(span);
                    ident
                }
                _ => continue,
            };

            let is_debug = is_debug_print(read);

            let tokens = if is_debug {
                // We cannot add colors to `Debug` formatted args, since they
                // might not support the `Display` trait, i.e. we cannot use `.to_string()`.
                quote! {
                    #ident = self.#ident
                }
            } else {
                quote! {
                    #ident = ::error_snippet::color_arg_hash(
                        ::std::string::ToString::to_string(&self.#ident)
                    )
                }
            };

            args.push(tokens);
        }

        let fmt_lit = LitStr::new(&fmt, span);

        quote! {
            format!( #fmt_lit, #(#args),* )
        }
    }

    fn read_ident(read: &mut &str) -> Ident {
        let mut ident = String::new();

        for (i, ch) in read.char_indices() {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => ident.push(ch),
                _ => {
                    *read = &read[i..];
                    break;
                }
            }
        }

        Ident::parse_any.parse_str(&ident).unwrap()
    }
}
