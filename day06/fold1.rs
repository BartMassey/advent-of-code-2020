pub trait Fold1Ext<T> {
    fn fold1<F>(&mut self, f: F) -> Option<T>
    where F: FnMut(T, T) -> T;
}

impl<T, I> Fold1Ext<T> for I
    where I: Iterator<Item = T>
{
    fn fold1<F>(&mut self, f: F) -> Option<T>
    where F: FnMut(T, T) -> T
    {
        // Cribbed from `std::iterator`.
        let first = self.next()?;
        Some(self.fold(first, f))
    }
}
