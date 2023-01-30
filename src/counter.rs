use proc_macro2::{Span, TokenStream, TokenTree};
use serde::Serialize;
use syn::{spanned::Spanned, visit::Visit, *};

#[derive(Default, Debug, Serialize)]
pub struct Counter {
    pub lines: usize,
    pub doc_lines: usize,
    pub test_lines: usize,
}

impl Counter {
    fn total(&self) -> usize {
        self.lines + self.test_lines + self.doc_lines
    }
}

fn has_test(tokens: TokenStream) -> bool {
    tokens.into_iter().any(|token| match token {
        TokenTree::Group(g) => has_test(g.stream()),
        TokenTree::Ident(i) => i.to_string() == "test",
        _ => false,
    })
}

// #[cfg(test)] or #[test]
fn is_test(attr: &Attribute) -> bool {
    if let Some(ident) = attr.path.get_ident() {
        match ident.to_string().as_str() {
            "test" => return true,
            "cfg" => {
                if has_test(attr.tokens.clone()) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn lines(span: Span) -> usize {
    span.end().line - span.start().line + 1
}

macro_rules! count {
    ($($method: ident, $ty: ident),*) => {
        $(
            fn $method(&mut self, i: &'ast $ty) {
                let lines = lines(i.span());
                if i.attrs.iter().any(is_test) {
                    self.test_lines += lines;
                } else {
                    self.lines += lines;
                }
            }
        )*
    }
}

impl<'ast> Visit<'ast> for Counter {
    fn visit_attribute(&mut self, i: &'ast Attribute) {
        if let Some(ident) = i.path.get_ident() {
            if ident.to_string() == "doc" {
                self.doc_lines += lines(ident.span());
            }
        }
        visit::visit_attribute(self, i);
    }

    count!(
        visit_item_const,
        ItemConst,
        visit_item_enum,
        ItemEnum,
        visit_item_extern_crate,
        ItemExternCrate,
        visit_item_fn,
        ItemFn,
        visit_item_foreign_mod,
        ItemForeignMod,
        visit_item_impl,
        ItemImpl,
        visit_item_macro,
        ItemMacro,
        visit_item_macro2,
        ItemMacro2,
        visit_item_mod,
        ItemMod,
        visit_item_static,
        ItemStatic,
        visit_item_struct,
        ItemStruct,
        visit_item_trait,
        ItemTrait,
        visit_item_trait_alias,
        ItemTraitAlias,
        visit_item_type,
        ItemType,
        visit_item_union,
        ItemUnion,
        visit_item_use,
        ItemUse
    );
}
