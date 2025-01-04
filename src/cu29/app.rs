use cu29::prelude::*;
use cu29_helpers::basic_copper_setup;

const SLAB_SIZE: Option<usize> = Some(150 * 1024 * 1024);

// NOTE: this will use the default config file in the current directory during compilation
// however, it will be overridden by the ron config string when the pipeline is started
#[copper_runtime(config = "bubbaloop.ron")]
struct CopperApp {}

pub struct CopperPipeline(pub CopperApp);

impl CopperPipeline {
    pub fn new(config: CuConfig) -> CuResult<Self> {
        // NOTE: this is a temporary solution to store the logger in the user's home directory
        let logger_dir = std::path::PathBuf::from(&format!("/home/{}", whoami::username()));
        let logger_path = logger_dir.join("bubbaloop.copper");
        debug!("Logger path: {}", path = &logger_path);

        let copper_ctx = basic_copper_setup(&logger_path, SLAB_SIZE, true, None)?;
        let application = CopperAppBuilder::new()
            .with_context(&copper_ctx)
            .with_config(config)
            .build()?;

        Ok(Self(application))
    }
}

impl std::ops::Deref for CopperPipeline {
    type Target = CopperApp;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CopperPipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
