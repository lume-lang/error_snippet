use proc_macro2::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, parse::Parser, Ident, LitStr};

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

            let tokens = quote! {
                #ident = self.#ident
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
