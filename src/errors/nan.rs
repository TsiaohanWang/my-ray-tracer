use super::MainErr;

pub trait F64NaNErr {
    fn from_msg(msg: String) -> Self;
}

impl F64NaNErr for MainErr {
    fn from_msg(msg: String) -> Self {
        Self::e(msg.as_str())
    }
}

pub fn check<E: F64NaNErr>(value: f64, val_details: &str) -> Result<f64, E> {
    if value.is_nan() {
        return Err(E::from_msg(
            "NaN found when checking ".to_string() + val_details,
        ));
    }

    Ok(value)
}
