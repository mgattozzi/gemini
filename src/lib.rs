use proc_macro::TokenStream;
use quote::quote;
use syn::{
  parse_macro_input,
  visit_mut::{self, VisitMut},
  Expr, ItemFn,
};

#[proc_macro_attribute]
pub fn gemini(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
  let input = parse_macro_input!(tokens as ItemFn);
  if let None = &input.sig.asyncness {
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
    if let Expr::Await(await_expr) = &node {
      *node = *await_expr.base.clone();
      return;
    }
    // Delegate to the default impl to visit nested expressions.
    visit_mut::visit_expr_mut(self, node);
  }
}
