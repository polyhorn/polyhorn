use std::io::Result;
use std::path::Path;

/// Rasterizes an SVG at the given source path with the given zoom and writes
/// the result to the given destination path.
pub fn rasterize(source: &Path, zoom: f32, destination: &Path) -> Result<()> {
    #[cfg(feature = "embed-svg")]
    {
        let tree = usvg::Tree::from_file(source, &usvg::Options::default()).unwrap();
        let image = resvg::render(&tree, usvg::FitTo::Zoom(zoom), None).unwrap();
        image.save_png(destination).unwrap();
        Ok(())
    }

    #[cfg(not(feature = "embed-svg"))]
    {
        use std::process::Command;

        assert!(Command::new("resvg")
            .arg("-z")
            .arg(format!("{}", zoom))
            .args(&[source, destination])
            .status()?
            .success());

        Ok(())
    }
}
