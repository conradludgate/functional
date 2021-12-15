use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse_macro_input, parse_quote,
    visit_mut::{visit_expr_mut, VisitMut},
    BinOp, Expr, ExprBinary, Item,
};

#[proc_macro_attribute]
pub fn pipe(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);
    // dbg!(&expr);
    Visitor.visit_item_mut(&mut item);
    item.into_token_stream().into()
}

struct Visitor;

impl VisitMut for Visitor {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        match i {
            Expr::Binary(ExprBinary {
                attrs,
                left,
                op,
                right,
            }) if matches!(op, BinOp::Shr(_)) => {
                for it in attrs {
                    self.visit_attribute_mut(it);
                }
                self.visit_expr_mut(&mut *left);
                self.visit_expr_mut(&mut *right);

                *i = parse_quote! { (#right)(#left) }
            }
            i => visit_expr_mut(self, i),
        }
    }
}
