/// An extension trait used to add the [TryStatefulFold::try_fold_with_state] function
/// to all iterators.
pub trait TryStatefulFold<Item> {
    /// Provided any type that implements [TryFoldable] as a type parameter and the
    /// [Iterator] will handle the rest.
    fn try_fold_with_state<Fold>(self) -> Result<Fold::Result, Fold::Error>
    where
        Fold: TryFoldable<Item>;
}

impl<Iter> TryStatefulFold<Iter::Item> for Iter
where
    Iter: Iterator + Sized,
{
    fn try_fold_with_state<Fold>(mut self) -> Result<Fold::Result, Fold::Error>
    where
        Fold: TryFoldable<Iter::Item>,
    {
        self.try_fold(Fold::init(), Fold::try_fold)
            .map(Fold::get_result)
    }
}

/// Defines the interface for a type which can be 'folded' into another type, but which may
/// also fail. If you need a non-fallible version, use [crate::fold::Foldable]
pub trait TryFoldable<Item>: Default {
    type Result;
    type Error;

    /// Defines how the state to be folded on should be initialized. Prefer
    /// implementing or deriving the [Default] trait instead of overriding the
    /// default implementation.
    fn init() -> Self {
        Default::default()
    }

    /// The fold function, identical to what you would write when using
    /// [Iterator::try_fold] directly.
    fn try_fold(self, item: Item) -> Result<Self, Self::Error>;

    /// Defines how to extract the final value from the state.
    fn get_result(self) -> Self::Result;
}
