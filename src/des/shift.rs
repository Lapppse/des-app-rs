use super::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ShiftDirection {
    Left,
    Right,
}

impl ShiftDirection {
    pub fn get_round_shift(&self, round: u8) -> Result<u8> {
        let round = match self {
            Self::Left => round,
            Self::Right => 16 - round + 1,
        };
        let round_shift = round * 2;
        match round {
            1 => Ok(round_shift - 1),
            (2..=8) => Ok(round_shift - 2),
            (9..=15) => Ok(round_shift - 3),
            16 => Ok(round_shift - 4),
            _ => Err(Error::InvalidRound(round)),
        }
    }
}

// FIXME: move to struct?
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ShiftSchemes {
    PC1,
    PC2,
}

impl ShiftSchemes {
    pub fn as_slice<'a>(&self) -> &'a [usize] {
        match self {
            Self::PC1 => [
                56, 48, 40, 32, 24, 16, 8, 0, 57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18,
                10, 2, 59, 51, 43, 35, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29, 21, 13,
                5, 60, 52, 44, 36, 28, 20, 12, 4, 27, 19, 11, 3,
            ]
            .as_slice(),
            Self::PC2 => [
                13, 16, 10, 23, 0, 4, 2, 27, 14, 5, 20, 9, 22, 18, 11, 3, 25, 7, 15, 6, 26, 19, 12,
                1, 40, 51, 30, 36, 46, 54, 29, 39, 50, 44, 32, 47, 43, 48, 38, 55, 33, 52, 45, 41,
                49, 35, 28, 31,
            ]
            .as_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_offset() -> Result<()> {
        let mut round = 1;
        let mut result = ShiftDirection::Left.get_round_shift(round)?;
        assert_eq!(result, 1);

        round = 2;
        result = ShiftDirection::Left.get_round_shift(round)?;
        assert_eq!(result, 2);

        round = 9;
        result = ShiftDirection::Left.get_round_shift(round)?;
        assert_eq!(result, 15);

        round = 16;
        result = ShiftDirection::Left.get_round_shift(round)?;
        assert_eq!(result, 28);
        Ok(())
    }
}
