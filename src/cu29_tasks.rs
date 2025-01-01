use cu29::prelude::*;
use kornia::io::stream::{
    video::{ImageFormat, VideoCodec, VideoWriter as KorniaVideoWriter},
    CameraCapture, V4L2CameraConfig,
};

pub type ImageRGBU8 = kornia::image::Image<u8, 3>;

#[derive(Clone)]
pub struct ImageRGBU8Msg {
    pub image: ImageRGBU8,
}

impl std::fmt::Debug for ImageRGBU8Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImageRGBU8Msg(size: {:?})", self.image.size())
    }
}

impl Default for ImageRGBU8Msg {
    fn default() -> Self {
        Self {
            image: ImageRGBU8::new([0, 0].into(), vec![]).unwrap(),
        }
    }
}

impl bincode::enc::Encode for ImageRGBU8Msg {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.image.rows(), encoder)?;
        bincode::Encode::encode(&self.image.cols(), encoder)?;
        bincode::Encode::encode(&self.image.as_slice(), encoder)?;
        Ok(())
    }
}

impl bincode::de::Decode for ImageRGBU8Msg {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let rows = bincode::Decode::decode(decoder)?;
        let cols = bincode::Decode::decode(decoder)?;
        let data = bincode::Decode::decode(decoder)?;
        let image = ImageRGBU8::new([rows, cols].into(), data)
            .map_err(|e| bincode::error::DecodeError::OtherString(e.to_string()))?;
        Ok(Self { image })
    }
}

pub struct VideoCapture {
    cap: CameraCapture,
}

impl VideoCapture {
    const DEFAULT_CAMERA_ID: u32 = 0;
    const DEFAULT_RES_ROWS: u32 = 480;
    const DEFAULT_RES_COLS: u32 = 640;
    const DEFAULT_FPS: u32 = 30;
}

impl Freezable for VideoCapture {}

impl<'cl> CuSrcTask<'cl> for VideoCapture {
    type Output = output_msg!('cl, ImageRGBU8Msg);

    fn new(config: Option<&ComponentConfig>) -> CuResult<Self>
    where
        Self: Sized,
    {
        let (camera_id, res_rows, res_cols, fps) = if let Some(config) = config {
            let camera_id = config
                .get::<u32>("camera_id")
                .unwrap_or(Self::DEFAULT_CAMERA_ID);
            let res_rows = config
                .get::<u32>("res_rows")
                .unwrap_or(Self::DEFAULT_RES_ROWS);
            let res_cols = config
                .get::<u32>("res_cols")
                .unwrap_or(Self::DEFAULT_RES_COLS);
            let fps = config.get::<u32>("fps").unwrap_or(Self::DEFAULT_FPS);
            (camera_id, res_rows, res_cols, fps)
        } else {
            (
                Self::DEFAULT_CAMERA_ID,
                Self::DEFAULT_RES_ROWS,
                Self::DEFAULT_RES_COLS,
                Self::DEFAULT_FPS,
            )
        };

        let cap = V4L2CameraConfig::new()
            .with_camera_id(camera_id)
            .with_fps(fps)
            .with_size([res_cols as usize, res_rows as usize].into())
            .build()
            .map_err(|e| CuError::new_with_cause("Failed to build camera", e))?;

        Ok(Self { cap })
    }

    fn start(&mut self, _clock: &RobotClock) -> CuResult<()> {
        self.cap
            .start()
            .map_err(|e| CuError::new_with_cause("Failed to start camera", e))
    }

    fn stop(&mut self, _clock: &RobotClock) -> CuResult<()> {
        self.cap
            .close()
            .map_err(|e| CuError::new_with_cause("Failed to stop camera", e))
    }

    fn process(&mut self, _clock: &RobotClock, output: Self::Output) -> CuResult<()> {
        let Some(image) = self
            .cap
            .grab()
            .map_err(|e| CuError::new_with_cause("Failed to grab image", e))?
        else {
            return Ok(());
        };

        output.set_payload(ImageRGBU8Msg { image });

        Ok(())
    }
}

pub struct VideoWriter {
    writer: Option<KorniaVideoWriter>,
}

impl VideoWriter {
    const DEFAULT_RES_ROWS: u32 = 480;
    const DEFAULT_RES_COLS: u32 = 640;
    const DEFAULT_FPS: u32 = 30;
}

impl Freezable for VideoWriter {}

impl<'cl> CuSinkTask<'cl> for VideoWriter {
    type Input = input_msg!('cl, ImageRGBU8Msg);

    fn new(config: Option<&ComponentConfig>) -> CuResult<Self>
    where
        Self: Sized,
    {
        // generate path file based on the current timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let path = format!("video_{}.mp4", timestamp);

        let (res_rows, res_cols, fps) = if let Some(config) = config {
            let res_cols = config
                .get::<u32>("res_cols")
                .unwrap_or(Self::DEFAULT_RES_COLS);
            let res_rows = config
                .get::<u32>("res_rows")
                .unwrap_or(Self::DEFAULT_RES_ROWS);
            let fps = config.get::<u32>("fps").unwrap_or(Self::DEFAULT_FPS);
            (res_rows, res_cols, fps)
        } else {
            (
                Self::DEFAULT_RES_ROWS,
                Self::DEFAULT_RES_COLS,
                Self::DEFAULT_FPS,
            )
        };

        let writer = KorniaVideoWriter::new(
            path,
            VideoCodec::H264,
            ImageFormat::Rgb8,
            fps as i32,
            [res_cols as usize, res_rows as usize].into(),
        )
        .map_err(|e| CuError::new_with_cause("Failed to create video writer", e))?;
        Ok(Self {
            writer: Some(writer),
        })
    }

    fn start(&mut self, _clock: &RobotClock) -> CuResult<()> {
        let Some(writer) = self.writer.as_mut() else {
            return Ok(());
        };

        writer
            .start()
            .map_err(|e| CuError::new_with_cause("Failed to start video writer", e))?;

        Ok(())
    }

    fn stop(&mut self, _clock: &RobotClock) -> CuResult<()> {
        let Some(writer) = self.writer.as_mut() else {
            return Ok(());
        };

        writer
            .close()
            .map_err(|e| CuError::new_with_cause("Failed to close video writer", e))?;

        self.writer = None; // drop the writer

        Ok(())
    }

    fn process(&mut self, _clock: &RobotClock, input: Self::Input) -> CuResult<()> {
        let Some(ImageRGBU8Msg { image }) = input.payload() else {
            return Ok(());
        };

        let Some(writer) = self.writer.as_mut() else {
            return Ok(());
        };

        writer
            .write(image)
            .map_err(|e| CuError::new_with_cause("Failed to write image", e))?;

        Ok(())
    }
}
