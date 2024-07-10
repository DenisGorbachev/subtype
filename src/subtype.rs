#[macro_export]
macro_rules! define_subtype {
    () => {
        use $crate::traits::transform::Transform;

        #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
        #[cfg_attr(feature = "derive_more", derive(derive_more::Deref, derive_more::From, derive_more::Into, derive_more::AsRef))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        #[repr(transparent)]
        pub struct Subtype<Value, Transformer: Transform<Value>> {
            #[deref]
            value: Value,
            phantom: PhantomData<Transformer>,
        }

        impl<Value, Transformer: Transform<Value>> Subtype<Value, Transformer> {
            pub fn new(value: impl Into<Value>) -> Result<Self, <Transformer as Transform<Value>>::Error> {
                Ok(Self {
                    value: Transformer::transform(value.into())?,
                    phantom: PhantomData,
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
    };
}
