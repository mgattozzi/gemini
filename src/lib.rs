use proc_macro::TokenStream;
use quote::quote;
use syn::{
  parse_macro_input,
  visit_mut::{self, VisitMut},
  Block, Expr, ExprBlock, ItemFn, Stmt,
};

#[proc_macro_attribute]
pub fn gemini(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
  let input = parse_macro_input!(tokens as ItemFn);
  if input.sig.asyncness.is_none() {
    panic!("The gemini macro requires that you put the attribute on an async function")
  }
  let mut sync = input.clone();
  sync.sig.asyncness = None;

  StripAwait.visit_block_mut(&mut sync.block);
  TokenStream::from(quote! {
    #[cfg(not(feature = "sync"))]
    #input

    #[cfg(feature = "sync")]
    #sync
  })
}

struct StripAwait;

impl VisitMut for StripAwait {
  fn visit_expr_mut(&mut self, node: &mut Expr) {
    match &node {
      Expr::Await(expr) => *node = *expr.base.clone(),
      Expr::Async(expr) => {
        let attrs = expr.attrs.clone();
        let mut block = expr.block.clone();
        self.visit_block_mut(&mut block);
        *node = Expr::Block(ExprBlock {
          attrs,
          block,
          label: None,
        });
      }
      // Delegate to the default impl to visit nested expressions.
      _ => visit_mut::visit_expr_mut(self, node),
    }
  }

  fn visit_block_mut(&mut self, node: &mut Block) {
    for stmt in &mut node.stmts {
      match stmt {
        Stmt::Expr(expr) | Stmt::Semi(expr, _) => (*self).visit_expr_mut(expr),
        _ => visit_mut::visit_stmt_mut(self, stmt),
      }
    }
  }
}
