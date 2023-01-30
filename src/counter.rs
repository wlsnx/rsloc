use bit_set::BitSet;
use proc_macro2::{Span, TokenStream, TokenTree};
use syn::{visit::Visit, *};

#[derive(Default, Debug)]
pub struct Counter {
    pub lines: BitSet,
    pub doc_lines: usize,
}

impl Counter {}

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

fn lines(span: &Span) -> usize {
    span.end().line - span.start().line + 1
}

macro_rules! count {
    ($($method: ident, $ty: ident),*) => {
        $(
            fn $method(&mut self, i: &'ast $ty) {
                if !i.attrs.iter().any(is_test) {
                    visit::$method(self, i);
                }
            }
        )*
    }
}

impl<'ast> Visit<'ast> for Counter {
    fn visit_attribute(&mut self, i: &'ast Attribute) {
        if let Some(ident) = i.path.get_ident() {
            if ident.to_string() == "doc" {
                self.doc_lines += lines(&ident.span());
            }
        }
        visit::visit_attribute(self, i);
    }

    fn visit_span(&mut self, i: &Span) {
        let start = i.start().line;
        let end = i.end().line;
        self.lines.insert(start);
        if end != start {
            self.lines.insert(end);
        }
    }

    fn visit_lit_str(&mut self, i: &'ast LitStr) {
        let span = i.span();
        (span.start().line..=span.end().line).for_each(|i| {
            self.lines.insert(i);
        });
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
