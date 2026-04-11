use async_trait::async_trait;
use stevessr_plugin_api::*;
use stevessr_plugin_api::context::PluginContext;
use stevessr_plugin_api::error::PluginError;
use stevessr_plugin_api::manifest::PluginManifest;

pub struct ExampleNativePlugin {
    manifest: PluginManifest,
}

impl ExampleNativePlugin {
    pub fn new() -> Self {
        let manifest_str = include_str!("../Plugin.toml");
        let manifest = PluginManifest::from_toml(manifest_str).expect("valid manifest");
        Self { manifest }
    }
}

#[async_trait]
impl Plugin for ExampleNativePlugin {
    fn manifest(&self) -> &PluginManifest { &self.manifest }

    async fn init(&mut self, ctx: &mut PluginContext) -> Result<(), PluginError> {
        ctx.logger.info("example native plugin initialized");
        Ok(())
    }

    async fn configure(&mut self, _ctx: &mut PluginContext) -> Result<(), PluginError> { Ok(()) }
    async fn activate(&mut self, _ctx: &mut PluginContext) -> Result<(), PluginError> { Ok(()) }
    async fn deactivate(&mut self, _ctx: &mut PluginContext) -> Result<(), PluginError> { Ok(()) }
    async fn destroy(&mut self) -> Result<(), PluginError> { Ok(()) }
}

declare_plugin!(ExampleNativePlugin, ExampleNativePlugin::new());
