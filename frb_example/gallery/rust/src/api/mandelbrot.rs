pub async fn draw_mandelbrot(
    image_size: Size,
    zoom_point: Point,
    scale: f64,
    num_threads: i32,
) -> anyhow::Result<Vec<u8>> {
    crate::ignore_me::mandelbrot_related::mandelbrot(image_size, zoom_point, scale, num_threads)
        .await
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
