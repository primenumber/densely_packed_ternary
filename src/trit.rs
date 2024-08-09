#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Trit(u8);

pub const T0: Trit = Trit(0);
pub const T1: Trit = Trit(1);
pub const T2: Trit = Trit(2);

impl TryFrom<u8> for Trit {
    type Error = ();
    fn try_from(x: u8) -> Result<Self, Self::Error> {
        if x < 3 {
            Ok(Trit(x))
        } else {
            Err(())
        }
    }
}

impl From<Trit> for u8 {
    fn from(x: Trit) -> Self {
        x.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_between_u8() {
        let t2 = Trit::try_from(2u8);
        assert_eq!(t2, Ok(Trit(2u8)));
        let invalid_trit = Trit::try_from(3u8);
        assert_eq!(invalid_trit, Err(()));
    }
}
