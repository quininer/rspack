use std::sync::Arc;

use rspack_core::{Compilation, rspack_sources::Source};
use rspack_hook::define_hook;
use rspack_util::allocative;

define_hook!(RealContentHashPluginUpdateHash: SeriesBail(compilation: &Compilation, assets: &[Arc<dyn Source>], old_hash: &str) -> String);

#[derive(Debug, Default)]
#[derive(allocative::Allocative)]
pub struct RealContentHashPluginHooks {
  #[allocative(skip)]
  pub update_hash: RealContentHashPluginUpdateHashHook,
}
