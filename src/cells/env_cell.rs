const ENV_CELL_MASK_HABITABLE: u8 = 0b1000000;
const ENV_CELL_MASK_OCCUPIED:  u8 = 0b10000000;

#[derive(Clone, Copy, Debug, Default)]
pub struct EnvironmentCell {
    pub habitable: bool,
    pub occupied: bool,
}

impl EnvironmentCell {
    pub fn new(habitable: bool, occupied: bool) -> Self {
        EnvironmentCell {
            habitable,
            occupied
        }
    }
}

impl From<u8> for EnvironmentCell {
    fn from(value: u8) -> Self {
        let habitable: bool = (value & ENV_CELL_MASK_HABITABLE) != 0b0;
        let occupied:  bool = (value & ENV_CELL_MASK_OCCUPIED) != 0b0;
        Self {
            habitable,
            occupied
        }
    }
}

impl From<EnvironmentCell> for u8 {
    fn from(value: EnvironmentCell) -> Self {
        let mut result = 0;
        if value.habitable {
            result = result | ENV_CELL_MASK_HABITABLE;
        }
        if value.occupied {
            result = result | ENV_CELL_MASK_OCCUPIED;
        }
        result
    }
}