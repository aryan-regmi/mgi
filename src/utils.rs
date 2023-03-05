pub(crate) fn screen_to_pixel(screen_size: (f32, f32), x: f32, y: f32) -> usize {
    let (w, h) = screen_size;

    let mut idx: usize = (4. * (h * x + y)) as usize;
    if idx >= (4. * h * w) as usize {
        idx = ((4. * h * w) - 4.) as usize;
    }

    idx
}
