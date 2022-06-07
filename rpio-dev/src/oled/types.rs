pub trait FrameBuffer {
    type Scan;
    type ScanOut;

    fn clear(&mut self);
    fn write_scan(&mut self, y: usize, scan: Self::Scan) -> bool;
    fn write(&mut self, y: usize, scans: &[Self::Scan]);
    fn read(&self) -> &[Self::ScanOut];

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn wrap_left(&mut self, shl: u32);
    fn wrap_range_left(&mut self, y0: usize, y1: usize, shl: u32);
    fn wrap_right(&mut self, shr: u32);
    fn wrap_range_right(&mut self, y0: usize, y1: usize, shr: u32);
}

pub trait Draw<W>: FrameBuffer {
    fn draw(&mut self, x: usize, y: usize, data: &[W]);
    fn blit(&mut self, x: usize, y: usize, data: &[W]);
}

pub trait Display {
    fn update(&mut self, fb: &[u128]);
}
