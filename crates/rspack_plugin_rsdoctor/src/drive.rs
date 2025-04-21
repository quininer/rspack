use rspack_hook::define_hook;
use rspack_util::allocative;

use crate::{
  RsdoctorAssetPatch, RsdoctorChunkGraph, RsdoctorModuleGraph, RsdoctorModuleIdsPatch,
  RsdoctorModuleSourcesPatch,
};

define_hook!(RsdoctorPluginModuleGraph: SeriesBail(data: &mut RsdoctorModuleGraph) -> bool);
define_hook!(RsdoctorPluginChunkGraph: SeriesBail(data: &mut RsdoctorChunkGraph) -> bool);
define_hook!(RsdoctorPluginModuleIds: SeriesBail(data: &mut RsdoctorModuleIdsPatch) -> bool);
define_hook!(RsdoctorPluginModuleSources: SeriesBail(data: &mut RsdoctorModuleSourcesPatch) -> bool);
define_hook!(RsdoctorPluginAssets: SeriesBail(data: &mut RsdoctorAssetPatch) -> bool);

#[derive(Debug, Default)]
#[derive(allocative::Allocative)]
pub struct RsdoctorPluginHooks {
  #[allocative(skip)]
  pub module_graph: RsdoctorPluginModuleGraphHook,
  #[allocative(skip)]
  pub chunk_graph: RsdoctorPluginChunkGraphHook,
  #[allocative(skip)]
  pub module_ids: RsdoctorPluginModuleIdsHook,
  #[allocative(skip)]
  pub module_sources: RsdoctorPluginModuleSourcesHook,
  #[allocative(skip)]
  pub assets: RsdoctorPluginAssetsHook,
}
