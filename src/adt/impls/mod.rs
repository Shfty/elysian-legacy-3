//! Function implementations for the ADT

mod apply;
mod chain;
mod fmap;
mod foldl;
mod foldr;
mod mappend;
mod mconcat;
mod mempty;
mod pure;
mod r#return;

pub use apply::*;
pub use chain::*;
pub use fmap::*;
pub use foldl::*;
pub use foldr::*;
pub use mappend::*;
pub use mconcat::*;
pub use mempty::*;
pub use pure::*;
pub use r#return::*;
