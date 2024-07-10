#[macro_export]
macro_rules! impl_new {
    ($transformer:ty, $oldtype:ty, impl$(<$($generics:tt),*>)? for $newtype:ty $(where $($where_clause:tt)*)?) => {
        impl$(<$($generics),*>)? $newtype $(where $($where_clause)*)? {
            pub fn new(value: impl Into<$oldtype>) -> Result<Self, <$transformer as $crate::traits::transform::Transform<$oldtype>>::Error> {
                let value = <$transformer as $crate::traits::transform::Transform<$oldtype>>::transform(value.into())?;
                Ok(Self(value))
            }

            pub fn set(&mut self, value: impl Into<$oldtype>) -> Result<(), <$transformer as $crate::traits::transform::Transform<$oldtype>>::Error> {
                let value = <$transformer as $crate::traits::transform::Transform<$oldtype>>::transform(value.into())?;
                self.0 = value;
                Ok(())
            }

            // pub fn mut(&mut self, f: impl FnOnce(&mut $oldtype))

            // try_mut
        }
    };
}

#[macro_export]
macro_rules! impl_try_from {
    (($($meta:meta),*)) => {
        #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
        #[derive(derive_more::Deref, derive_more::From, derive_more::Into, derive_more::AsRef)]
        $(#[$meta])*
        #[repr(transparent)]
        pub struct Subtype<Value, Transformer: Transform<Value>> {
            #[deref]
            #[as_ref]
            value: Value,
            phantom: std::marker::PhantomData<Transformer>,
        }

        impl<Value, Transformer: Transform<Value>> Subtype<Value, Transformer> {
            pub fn new(value: impl Into<Value>) -> Result<Self, <Transformer as Transform<Value>>::Error> {
                Ok(Self {
                    value: Transformer::transform(value.into())?,
                    phantom: std::marker::PhantomData,
                })
            }

            /// This method is only available if `Value` implements `Copy`.
            ///
            /// Use [`.as_ref()`](AsRef::as_ref) to get the value by reference if it doesn't implement `Copy`.
            pub fn get(&self) -> Value
            where
                Value: Copy,
            {
                self.value
            }

            pub fn set(&mut self, value: impl Into<Value>) -> Result<(), <Transformer as Transform<Value>>::Error> {
                let value = Transformer::transform(value.into())?;
                self.value = value;
                Ok(())
            }
        }

        // impl<Value, Transformer: Transform<Value>> TryFrom<Value> for Subtype<Value, Transformer> {
        //     type Error = <Transformer as Transform<Value>>::Error;
        //
        //     fn try_from(value: Value) -> Result<Self, Self::Error> {
        //         Self::new(value)
        //     }
        // }
        //
        // impl<'a, Value: Clone, Transformer: Transform<Value>> TryFrom<&'a Value> for Subtype<Value, Transformer> {
        //     type Error = <Transformer as Transform<Value>>::Error;
        //
        //     fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        //         Self::new(value.clone())
        //     }
        // }
    };
}
