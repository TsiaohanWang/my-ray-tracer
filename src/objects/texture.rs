#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum OpaqueMaterial {
    Plastic,
    Fabric,
    Rubber,
    Wood,
    Metal,
    Null
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct OpaqueTexture {
    /// 材质颜色：RGBA 格式
    color: (u8, u8, u8, u8),
    /// 反射率
    reflectance: f64,
    /// 材料
    material: OpaqueMaterial,
}

impl OpaqueTexture {
    fn new_from(color_tuple: (u8, u8, u8, u8), reflectance: f64, material: OpaqueMaterial) -> Self {
        Self { color: color_tuple, reflectance, material }
    }
}