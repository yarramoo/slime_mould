
#[derive(Default, Clone, Copy, Debug)]
pub struct TrailCell {
    pub residue: f32,
}

impl From<&TrailCell> for [u8; 3] {
    fn from(value: &TrailCell) -> Self {
        [0, 0, (value.residue * 100.) as u8]
    }
}