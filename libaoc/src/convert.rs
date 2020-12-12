use std::convert::TryInto;
use std::fmt::Debug;

pub trait ConvertInto<U> {
    fn convert_into(self) -> U;
}

impl<T, U> ConvertInto<U> for T
where
    T: TryInto<U>,
    <T as TryInto<U>>::Error: Debug,
{
    fn convert_into(self) -> U {
        self.try_into().unwrap()
    }
}
