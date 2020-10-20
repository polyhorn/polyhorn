use polyhorn_android_sys::{Color, Env};

pub trait Convert<T> {
    fn convert(self, env: &Env) -> T;
}

impl Convert<Color> for polyhorn_ui::color::Color {
    fn convert(self, _env: &Env) -> Color {
        // TODO: add support for wide-gamut colors.
        let srgb = self.to_srgb();
        Color::device_rgb(srgb.red, srgb.green, srgb.blue, srgb.alpha)
    }
}
