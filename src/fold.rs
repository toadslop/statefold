/// Defines the interface for a type which can be 'folded' into another type.
pub trait Foldable<Item>: Default {
    type Result;

    /// Defines how the state to be folded on should be initialized. Prefer
    /// implementing or deriving the [Default] trait instead of overriding the
    /// default implementation.
    fn init() -> Self {
        Default::default()
    }

    /// The fold function, identical to what you would write when using
    /// [Iterator::fold] directly.
    fn fold(self, item: Item) -> Self;

    /// Defines how to extract the final value from the state.
    fn get_result(self) -> Self::Result;
}

/// An extension trait used to add the [StatefulFold::fold_with_state] function
/// to all iterators.
pub trait StatefulFold<Item> {
    /// Provided any type that implements [Foldable] as a type parameter and the
    /// [Iterator] will handle the rest.
    fn fold_with_state<Fold>(self) -> Fold::Result
    where
        Fold: Foldable<Item>;
}

impl<Iter> StatefulFold<Iter::Item> for Iter
where
    Iter: Iterator + Sized,
{
    fn fold_with_state<Fold>(self) -> Fold::Result
    where
        Fold: Foldable<Iter::Item>,
    {
        self.fold(Fold::init(), Fold::fold).get_result()
    }
}
