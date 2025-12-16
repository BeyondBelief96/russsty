/// A view into a 2D pixel buffer.
///
/// Wraps a 1D slice with width/height metadata to enable safe 2D pixel access.
/// This is a borrowed view, not an owning type - it's meant to be created
/// temporarily when you need to pass buffer + dimensions together.
pub(crate) struct FrameBuffer<'a> {
    data: &'a mut [u32],
    width: u32,
    height: u32,
}

impl<'a> FrameBuffer<'a> {
    /// Create a new FrameBuffer view from a buffer slice and dimensions.
    ///
    /// # Panics
    /// Panics if `buffer.len() != width * height`
    pub fn new(data: &'a mut [u32], width: u32, height: u32) -> Self {
        debug_assert_eq!(
            data.len(),
            (width * height) as usize,
            "Buffer size doesn't match dimensions"
        );
        Self { data, width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set a pixel at (x, y) to the given color.
    /// Silently ignores out-of-bounds coordinates.
    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.data[(y as u32 * self.width + x as u32) as usize] = color;
        }
    }

    /// Get the color at (x, y), or None if out of bounds.
    #[inline]
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<u32> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Some(self.data[(y as u32 * self.width + x as u32) as usize])
        } else {
            None
        }
    }

    /// Fill a horizontal span of pixels. More efficient than calling set_pixel in a loop.
    /// Automatically clips to buffer bounds.
    #[inline]
    pub fn fill_scanline(&mut self, y: i32, x_start: i32, x_end: i32, color: u32) {
        if y < 0 || y >= self.height as i32 {
            return;
        }

        let x_start = x_start.max(0) as u32;
        let x_end = (x_end as u32).min(self.width - 1);

        if x_start > x_end {
            return;
        }

        let row_offset = y as u32 * self.width;
        for x in x_start..=x_end {
            self.data[(row_offset + x) as usize] = color;
        }
    }
}
