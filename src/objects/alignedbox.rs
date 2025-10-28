#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct AxisAlignedBox {
    x_axis: (f64, f64),
    y_axis: (f64, f64),
    z_axis: (f64, f64)
}
