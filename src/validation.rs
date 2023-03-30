pub fn greater_than_zero(s: &str) -> Result<f32, String> {
    let n: f32 = s.parse().map_err(|_| "Invalid float")?;

    if !(n > 0.0) {
        return Err("Should be greater than zero".into());
    }

    Ok(n)
}
