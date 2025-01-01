use cu29::prelude::*;
use cu29_helpers::basic_copper_setup;

const SLAB_SIZE: Option<usize> = Some(1500 * 1024 * 1024);

#[copper_runtime(config = "bubbaloop.ron")]
struct CopperApp {}

pub struct CopperPipeline(pub CopperApp);

impl CopperPipeline {
    pub fn new() -> CuResult<Self> {
        let tmp_dir = tempfile::TempDir::new().expect("could not create a tmp dir");
        let logger_path = tmp_dir.path().join("bubbaloop.copper");
        debug!("Logger path: {}", path = &logger_path);

        let copper_ctx = basic_copper_setup(&logger_path, SLAB_SIZE, true, None)?;
        let application = CopperAppBuilder::new().with_context(&copper_ctx).build()?;

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
