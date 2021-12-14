use functional_syn::{
    parse_macro_input, parse_quote,
    visit_mut::{visit_expr_mut, VisitMut},
    BinOp, Expr, ExprBinary, Stmt, Block, parse::Parse, Result,
};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn pipe(input: TokenStream) -> TokenStream {
    let Stmts { mut stmts } = parse_macro_input!(input as Stmts);
    // dbg!(&expr);
    Visitor.visit(&mut stmts);
    quote!{ {#(#stmts)*} }.into()
}

struct Stmts {
    pub stmts: Vec<Stmt>,
}

impl Parse for Stmts {
    fn parse(input: functional_syn::parse::ParseStream) -> Result<Self> {
        Ok(Stmts {
            stmts: Block::parse_within(input)?,
        })
    }
}

struct Visitor;

impl Visitor {
    fn visit(&mut self, stmts: &mut [Stmt]) {
        stmts.iter_mut().for_each(|stmt| self.visit_stmt_mut(stmt));
    }
}

impl VisitMut for Visitor {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        match i {
            Expr::Binary(ExprBinary {
                attrs,
                left,
                op,
                right,
            }) if matches!(op, BinOp::Pipe(_)) => {
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
