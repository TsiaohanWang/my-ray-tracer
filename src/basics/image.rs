use core::f64;
use std::{
    cmp::Ordering,
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
};

use crate::errors::{MainErr, nan};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
/// 浮点 RGB 取值介于 0.0 到 1.0
pub struct ImgPixel {
    float_r: f64,
    float_g: f64,
    float_b: f64,
}

impl ImgPixel {
    pub fn new_from(float_r: f64, float_g: f64, float_b: f64) -> Result<Self, Box<dyn Error>> {
        if float_r > 1.0 || float_r < 0.0 {
            return Err(Box::new(ImageErr::InvalidRgbInputErr));
        }
        if float_g > 1.0 || float_g < 0.0 {
            return Err(Box::new(ImageErr::InvalidRgbInputErr));
        }
        if float_b > 1.0 || float_b < 0.0 {
            return Err(Box::new(ImageErr::InvalidRgbInputErr));
        }
        Ok(Self {
            float_r: nan::check::<MainErr>(float_r, "ImgPixel::new_from")?,
            float_g: nan::check::<MainErr>(float_g, "ImgPixel::new_from")?,
            float_b: nan::check::<MainErr>(float_b, "ImgPixel::new_from")?,
        })
    }

    /// 将当前 `ImgPixel` 的浮点 RGB 值安全转换为 `u8` 三元组
    pub fn scale_rgb(&self) -> (u8, u8, u8) {
        let ir: u8 = match (self.get_r() * FLOAT_RGB_INTO_INT_SCALE).round() as u16 {
            256_u16 => 255_u8,
            res => res as u8,
        };
        let ig: u8 = match (self.get_g() * FLOAT_RGB_INTO_INT_SCALE).round() as u16 {
            256_u16 => 255_u8,
            res => res as u8,
        };
        let ib: u8 = match (self.get_b() * FLOAT_RGB_INTO_INT_SCALE).round() as u16 {
            256_u16 => 255_u8,
            res => res as u8,
        };

        (ir, ig, ib)
    }

    /// 重设 `ImgPixel` 的浮点 RGB 值
    ///
    /// 当输入的浮点值不在规定范围（`[0..=1]`）内时，
    /// 返回 `ImageErr::InvalidRgbInputErr`
    ///
    /// 当输入的浮点值为 `f64::NAN` 时，
    /// 通过 `nan::check` 返回实现 `F64NaNErr` trait 的错误（即 `MainErr`）
    pub fn set(
        &mut self,
        float_r: f64,
        float_g: f64,
        float_b: f64,
    ) -> Result<&mut Self, Box<dyn Error>> {
        if float_r > 1.0 || float_r < 0.0 {
            return Err(Box::new(ImageErr::InvalidRgbInputErr));
        }
        if float_g > 1.0 || float_g < 0.0 {
            return Err(Box::new(ImageErr::InvalidRgbInputErr));
        }
        if float_b > 1.0 || float_b < 0.0 {
            return Err(Box::new(ImageErr::InvalidRgbInputErr));
        }
        self.float_r = nan::check::<MainErr>(float_r, "ImgPixel::set")?;
        self.float_g = nan::check::<MainErr>(float_g, "ImgPixel::set")?;
        self.float_b = nan::check::<MainErr>(float_b, "ImgPixel::set")?;
        Ok(self)
    }

    pub fn set_r(&mut self, float_r: f64) -> Result<&mut Self, Box<dyn Error>> {
        if float_r > 1.0 || float_r < 0.0 {
            return Err(Box::new(ImageErr::InvalidRgbInputErr));
        }
        self.float_r = nan::check::<MainErr>(float_r, "ImgPixel::set_r")?;
        Ok(self)
    }

    pub fn set_g(&mut self, float_g: f64) -> Result<&mut Self, Box<dyn Error>> {
        if float_g > 1.0 || float_g < 0.0 {
            return Err(Box::new(ImageErr::InvalidRgbInputErr));
        }
        self.float_g = nan::check::<MainErr>(float_g, "ImgPixel::set_g")?;
        Ok(self)
    }

    pub fn set_b(&mut self, float_b: f64) -> Result<&mut Self, Box<dyn Error>> {
        if float_b > 1.0 || float_b < 0.0 {
            return Err(Box::new(ImageErr::InvalidRgbInputErr));
        }
        self.float_b = nan::check::<MainErr>(float_b, "ImgPixel::set_b")?;
        Ok(self)
    }

    pub fn get_r(&self) -> f64 {
        self.float_r
    }

    pub fn get_g(&self) -> f64 {
        self.float_g
    }

    pub fn get_b(&self) -> f64 {
        self.float_b
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Img {
    width: usize,
    height: usize,
    pixels: Vec<ImgPixel>,
}

impl Img {
    /// 从指定宽高参数 `w`、`h` 创建一个新的 `Img`
    ///
    /// 当宽高参数含 `0` 时返回 `ImageErr::InvalidImgParamErr`
    pub fn new_from(w: usize, h: usize) -> Result<Self, Box<dyn Error>> {
        if w == 0 || h == 0 {
            return Err(Box::new(ImageErr::InvalidImgParamErr));
        }
        Ok(Self {
            width: w,
            height: h,
            pixels: Vec::with_capacity(w * h),
        })
    }

    pub fn get_w(&self) -> usize {
        self.width
    }

    pub fn get_h(&self) -> usize {
        self.height
    }

    /// 在 `Img` 中通过浮点 RGB 参数创建并添加新的 `ImgPixel`
    ///
    /// 添加方式为按行添加，一行满后添加下一行
    ///
    /// 若浮点 RGB 参数不符合要求会返回 ImgPixel::new_from 的返回类型
    pub fn append(
        &mut self,
        float_r: f64,
        float_g: f64,
        float_b: f64,
    ) -> Result<(), Box<dyn Error>> {
        self.pixels
            .push(ImgPixel::new_from(float_r, float_g, float_b)?);
        Ok(())
    }

    //     /// 封装版 `Img::append`，
    //     /// 使其能够预先处理 `ImageErr`，并将其他类型错误重新返回
    //     pub fn __append(
    //         &mut self,
    //         float_r: f64,
    //         float_g: f64,
    //         float_b: f64,
    //     ) -> Result<(), Box<dyn Error>> {
    //         if let Err(e) = self.append(float_r, float_g, float_b) {
    //             if let Some(image_err) = e.downcast_ref::<ImageErr>() {
    //                 image_err.handle();
    //             } else {
    //                 return Err(e);
    //             }
    //         }
    //
    //         Ok(())
    //     }

    /// 返回第 `line` 行（1 索引）、第 `col` 列（1 索引）处对应的 `ImgPixel`
    ///
    /// 如果索引本身无效，返回 `ImageErr::InvalidPixelIdxErr`
    ///
    /// 如果索引对应位置没有 `ImgPixel`，返回 `ImageErr::InvalidPixelIdxErr`
    pub fn index_of(&self, line: usize, col: usize) -> Result<ImgPixel, Box<dyn Error>> {
        if line <= 0 || col <= 0 {
            return Err(Box::new(ImageErr::InvalidPixelIdxErr));
        }
        if line * col > self.pixels.len() {
            return Err(Box::new(ImageErr::InvalidPixelIdxErr));
        }

        Ok(self.pixels[line * col - 1])
    }

    /// 返回最后一个 `ImgPixel` 对应的行（1 索引）、列（1索引）和一个 std::cmp::Ordering
    ///
    /// - 若当前 `Img` 所含 `ImgPixel` 少于参数 `width * height` 则返回 `Ordering::Less`
    /// - 若相等则返回 `Ordering::Equal`
    /// - 若大于则返回 `Ordering::Greater`
    ///
    /// 若返回 `(0, 0)` 表明当前 `Img` 不含任何 `ImgPixel`
    pub fn last(&self) -> (usize, usize, Ordering) {
        let line: usize = self.pixels.len() / self.get_w() + 1;
        let col: usize = self.pixels.len() % self.get_w();

        let w: usize = self.get_w();
        let h: usize = self.get_h();

        let check_res: Ordering = self.pixels.len().cmp(&(w * h));

        (line, col, check_res)
    }

    /// 私有方法，仅用于检查当前 `Img` 是否满足所含 `ImgPixel` 数等于 `width * height`
    ///
    /// 不满足时输出提示信息并返回 `ImageErr::InvalidPixelsErr`
    fn check(&self) -> Result<(), Box<dyn Error>> {
        let pixel_num: usize = self.get_w() * self.get_h();
        if pixel_num != self.pixels.len() {
            println!(
                "Invalid {}*{} image with {} pixel(s)!",
                self.get_w(),
                self.get_h(),
                self.pixels.len()
            );
            return Err(Box::new(ImageErr::InvalidPixelsErr));
        }
        Ok(())
    }

    /// 创建 `Img` 对应的 `image_output.ppm` 文件
    ///
    /// 此方法会尝试创建文件（可能返回错误）并写入内容（可能返回错误）
    ///
    /// 写入某一像素的 RGB 值时会将浮点 RGB 值比例缩放到 0～255，比例参数为 `FLOAT_RGB_INTO_INT_SCALE`
    ///
    /// 若当前 `Img` 不满足所含 `ImgPixel` 数等于 `width * height`，会返回 `Img::check` 的返回类型
    pub fn produce(&self) -> Result<(), Box<dyn Error>> {
        self.check()?;

        let f = File::create(IMAGE_OUTPUT_PATH)?;
        let mut writer = BufWriter::new(f);
        writeln!(writer, "P3\n{} {}\n255\n", self.width, self.height)?;
        for p in self {
            let (ir, ig, ib) = p.scale_rgb();
            writeln!(writer, "{} {} {}", ir, ig, ib)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ImageErr {
    /// 输入无效的 RGB 浮点值
    InvalidRgbInputErr,
    /// 输入无效的 `Img` 宽/高参数
    InvalidImgParamErr,
    /// `Img` 所含 `ImgPixel` 不满足 `Img` 宽高
    InvalidPixelsErr,
    /// 访问的索引不在当前 `Img` 所含 `ImgPixel` 的范围
    InvalidPixelIdxErr,
}

impl Display for ImageErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageErr::InvalidRgbInputErr => write!(f, "invalid rgb float value"),
            ImageErr::InvalidImgParamErr => {
                write!(f, "invalid image parameter (width or height)")
            }
            ImageErr::InvalidPixelsErr => write!(
                f,
                "existing amount of pixel(s) doesn't satisfy width * height"
            ),
            ImageErr::InvalidPixelIdxErr => {
                write!(f, "invalid index of current image pixel(s)")
            }
        }
    }
}

impl Error for ImageErr {}

impl ImageErr {
    pub fn handle(&self) {
        eprintln!("[Image Error] {}", self);
    }
}

#[derive(Debug, Clone)]
pub struct ImgPixelIter {
    index: usize,
    pixels: Vec<ImgPixel>,
}

impl Iterator for ImgPixelIter {
    type Item = ImgPixel;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.pixels.len() {
            return None;
        }
        self.index += 1;
        return Some(self.pixels[self.index - 1]);
    }
}

impl IntoIterator for &Img {
    type IntoIter = ImgPixelIter;
    type Item = ImgPixel;
    fn into_iter(self) -> Self::IntoIter {
        ImgPixelIter {
            index: 0,
            pixels: self.pixels.clone(),
        }
    }
}

const FLOAT_RGB_INTO_INT_SCALE: f64 = 255.999;
const IMAGE_OUTPUT_PATH: &str = "image_output.ppm";
