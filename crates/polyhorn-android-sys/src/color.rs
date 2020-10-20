pub enum Color {
    Unmanaged(i32),
    Managed(i64),
}

impl Color {
    /// Returns an unmanaged RGB color.
    pub fn device_rgb(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        let (red, green, blue, alpha) = (
            (red * 255.0) as u64,
            (green * 255.0) as u64,
            (blue * 255.0) as u64,
            (alpha * 255.0) as u64,
        );

        Color::Unmanaged((0u64 | (alpha << 24) | (red << 16) | (green << 8) | (blue << 0)) as i32)
    }
}
