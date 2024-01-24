use crate::DISPLAY_DIMS;

pub fn pos_to_idx((x, y): (usize, usize)) -> usize {
    (y * DISPLAY_DIMS.0) + x
}
pub fn idx_to_pos(idx: usize) -> (usize, usize) {
    (idx % DISPLAY_DIMS.0, idx / DISPLAY_DIMS.0)
}
