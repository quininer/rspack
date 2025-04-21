use std::ptr::NonNull;

use rspack_core::{ChunkUkey, Compilation, CompilationId};
use rspack_hook::define_hook;
use rspack_util::allocative;

#[derive(Debug, Clone)]
pub struct CreateScriptData {
  pub code: String,
  pub chunk: RuntimeModuleChunkWrapper,
}

#[derive(Debug, Clone)]
pub struct LinkPreloadData {
  pub code: String,
  pub chunk: RuntimeModuleChunkWrapper,
}

#[derive(Debug, Clone)]
pub struct LinkPrefetchData {
  pub code: String,
  pub chunk: RuntimeModuleChunkWrapper,
}

#[derive(Debug, Clone)]
pub struct RuntimeModuleChunkWrapper {
  pub chunk_ukey: ChunkUkey,
  pub compilation_id: CompilationId,
  pub compilation: NonNull<Compilation>,
}

unsafe impl Send for RuntimeModuleChunkWrapper {}

define_hook!(RuntimePluginCreateScript: SeriesWaterfall(data: CreateScriptData) -> CreateScriptData);
define_hook!(RuntimePluginLinkPreload: SeriesWaterfall(data: LinkPreloadData) -> LinkPreloadData);
define_hook!(RuntimePluginLinkPrefetch: SeriesWaterfall(data: LinkPrefetchData) -> LinkPrefetchData);

#[derive(Debug, Default)]
#[derive(allocative::Allocative)]
pub struct RuntimePluginHooks {
  #[allocative(skip)]
  pub create_script: RuntimePluginCreateScriptHook,
  #[allocative(skip)]
  pub link_preload: RuntimePluginLinkPreloadHook,
  #[allocative(skip)]
  pub link_prefetch: RuntimePluginLinkPrefetchHook,
}
