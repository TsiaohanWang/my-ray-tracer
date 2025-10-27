use core::f64;
use std::{
    cmp::Ordering, error::Error, fmt::Display, fs::File, io::{BufWriter, Write}
};

use crate::errors::{MainErr, nan::check};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
/// 浮点 RGB 取值介于 0.0 到 1.0
pub struct ImgPixel {
    float_r: f64,
    float_g: f64,
    float_b: f64,
}

impl ImgPixel {
    pub fn new_from(float_r: f64, float_g: f64, float_b: f64) -> Result<Self, ImageErr> {
        if float_r > 1.0 || float_r < 0.0 {
            return Err(ImageErr::InvalidRgbInputErr);
        }
        if float_g > 1.0 || float_g < 0.0 {
            return Err(ImageErr::InvalidRgbInputErr);
        }
        if float_b > 1.0 || float_b < 0.0 {
            return Err(ImageErr::InvalidRgbInputErr);
        }
        Ok(Self {
            float_r: check::<MainErr>(float_r, "ImgPixel::new_from").unwrap(),
            float_g: check::<MainErr>(float_g, "ImgPixel::new_from").unwrap(),
            float_b: check::<MainErr>(float_b, "ImgPixel::new_from").unwrap(),
        })
    }

    pub fn set(&mut self, float_r: f64, float_g: f64, float_b: f64) -> &mut Self {
        self.float_r = check::<MainErr>(float_r, "ImgPixel::set").unwrap();
        self.float_g = check::<MainErr>(float_g, "ImgPixel::set").unwrap();
        self.float_b = check::<MainErr>(float_b, "ImgPixel::set").unwrap();
        self
    }

    pub fn set_r(&mut self, float_r: f64) -> &mut Self {
        self.float_r = check::<MainErr>(float_r, "ImgPixel::set_r").unwrap();
        self
    }

    pub fn set_g(&mut self, float_g: f64) -> &mut Self {
        self.float_g = check::<MainErr>(float_g, "ImgPixel::set_g").unwrap();
        self
    }

    pub fn set_b(&mut self, float_b: f64) -> &mut Self {
        self.float_b = check::<MainErr>(float_b, "ImgPixel::set_b").unwrap();
        self
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
    pub fn new_from(w: usize, h: usize) -> Result<Self, ImageErr> {
        if w == 0 || h == 0 {
            return Err(ImageErr::InvalidImgParamErr);
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
    pub fn append(&mut self, float_r: f64, float_g: f64, float_b: f64) -> Result<(), ImageErr> {
        self.pixels
            .push(ImgPixel::new_from(float_r, float_g, float_b)?);
        Ok(())
    }

    /// 返回第 `line` 行（1 索引）、第 `col` 列（1 索引）处对应的 `ImgPixel`
    /// 
    /// 如果索引本身无效，返回 `ImageErr::InvalidPixelIdxErr`
    /// 
    /// 如果索引对应位置没有 `ImgPixel`，返回 `ImageErr::InvalidPixelIdxErr`
    pub fn index_of(&self, line: usize, col: usize) -> Result<ImgPixel, ImageErr> {
        if line <= 0 || col <= 0 {
            return Err(ImageErr::InvalidPixelIdxErr);
        }
        if line * col > self.pixels.len() {
            return Err(ImageErr::InvalidPixelIdxErr);
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
    fn check(&self) -> Result<(), ImageErr> {
        let pixel_num: usize = self.get_w() * self.get_h();
        if pixel_num != self.pixels.len() {
            println!(
                "Invalid {}*{} image with {} pixel(s)!",
                self.get_w(),
                self.get_h(),
                self.pixels.len()
            );
            return Err(ImageErr::InvalidPixelsErr);
        }
        Ok(())
    }

    /// 创建 `Img` 对应的 `.ppm` 格式文件
    /// 
    /// 此方法会尝试创建文件（可能返回错误）并写入内容（可能返回错误）
    /// 
    /// 写入某一像素的 RGB 值时会将浮点 RGB 值比例缩放到 0～255，比例参数为 `FLOAT_RGB_INTO_INT_SCALE`
    /// 
    /// 若当前 `Img` 不满足所含 `ImgPixel` 数等于 `width * height`，会返回 `Img::check` 的返回类型
    pub fn produce(&self) -> Result<(), Box<dyn Error>> {
        self.check().unwrap();

        let f = File::create("foo.txt")?;
        let mut writer = BufWriter::new(f);
        writeln!(writer, "P3\n{} {}\n255\n", self.width, self.height)?;
        for p in self {
            writeln!(
                writer,
                "{} {} {}",
                p.get_r() * FLOAT_RGB_INTO_INT_SCALE,
                p.get_g() * FLOAT_RGB_INTO_INT_SCALE,
                p.get_b() * FLOAT_RGB_INTO_INT_SCALE
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
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
            ImageErr::InvalidRgbInputErr => write!(f, "Image Error: invalid rgb float value"),
            ImageErr::InvalidImgParamErr => write!(f, "Image Error: invalid image parameter (width or height)"),
            ImageErr::InvalidPixelsErr => write!(f, "Image Error: existing amount of pixel(s) doesn't satisfy width * height"),
            ImageErr::InvalidPixelIdxErr => write!(f, "Image Error: invalid index of current image pixel(s)"),
        }
    }
}

#[derive(Debug)]
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
        return Some(self.pixels[self.index]);
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

const FLOAT_RGB_INTO_INT_SCALE: f64 = 255.99999;
