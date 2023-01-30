use bit_set::BitSet;
use proc_macro2::{TokenStream, TokenTree};
use syn::{spanned::Spanned, visit::Visit, *};

#[derive(Default, Debug)]
pub struct Counter {
    doc_lines: usize,
    set: BitSet,
}

impl Counter {
    pub fn lines(&self) -> usize {
        self.set.len() - self.doc_lines
    }
}

macro_rules! lit {
    ($($method: ident, $ty: ident),*) => {
        $(
            fn $method(&mut self, i: &'ast $ty) {
                (i.span().start().line..=i.span().end().line).for_each(|i| {
                    self.set.insert(i);
                });
                visit::$method(self, i);
            }
        )*
    }
}

fn has_test(tokens: TokenStream) -> bool {
    for token in tokens {
        match token {
            TokenTree::Group(g) => {
                if has_test(g.stream()) {
                    return true;
                }
            }
            TokenTree::Ident(i) => {
                if i.to_string() == "test" {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

macro_rules! test {
    ($($method: ident, $ty: ident),*) => {
        $(
            fn $method(&mut self, i: &'ast $ty) {
                for attr in i.attrs.iter() {
                    if let Some(ident) = attr.path.get_ident() {
                        match ident.to_string().as_str() {
                            "test" => return,
                            "cfg" => {
                                if has_test(attr.tokens.clone()) {
                                    return;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                visit::$method(self, i);
            }
        )*
    }
}

impl<'ast> Visit<'ast> for Counter {
    fn visit_attribute(&mut self, i: &'ast Attribute) {
        if let Some(ident) = i.path.get_ident() {
            if ident.to_string() == "doc" {
                self.doc_lines += i.span().end().line - i.span().start().line + 1;
            }
        }
        visit::visit_attribute(self, i);
    }

    fn visit_span(&mut self, i: &proc_macro2::Span) {
        let start_line = i.start().line;
        let end_line = i.end().line;
        self.set.insert(start_line);
        if start_line != end_line {
            self.set.insert(end_line);
        }
        visit::visit_span(self, i);
    }

    lit!(visit_lit_str, LitStr, visit_lit_byte_str, LitByteStr);

    test!(visit_item_fn, ItemFn, visit_item_mod, ItemMod);
}
