macro_rules! bitflag_combination {
    ($($n:ident)|+ $(,)?) => {
        0 $(
            | Self::$n.bits()
        )+
    };

    ($ty:ty, $($n:ident)|+ $(,)?) => {
       <$ty>::from_bits_retain(
           0 $(
             | <$ty>::$n.bits()
           )+
       )
    };
}
pub(crate) use bitflag_combination;