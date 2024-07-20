use subtype::checkers::Empty;
use subtype::transformers::trim::Trim;

#[macro_export]
macro_rules! define_custom {
    {
        $(#[$meta:meta])*
        $vis:vis $wrapper:ident: $inner:ty;
        $(adjust $adjust:ty;)?
        $(ensure $ensure:ty;)?
        $(validate($err:ty) $validate:expr;)?
        $(plugins: [$($plugin:path),+ $(,)?];)?
    } => {
        prae::define! {
            $(#[$meta])*
            $vis $wrapper: $inner;
            $(adjust <$adjust as subtype::traits::adjust::Adjust<$inner>>::adjust;)?
            $(ensure <$ensure as subtype::traits::check::Check<$inner>>::check;)?
            $(validate($err) $validate;)?
            $(plugins: [$($plugin),+ $(,)?];)?
        }
    };
}

#[macro_export]
macro_rules! define_string {
    {
        $(#[$meta:meta])*
        $vis:vis $wrapper:ident: String;
        $(adjust $adjust:ty;)?
        $(ensure $ensure:ty;)?
        $(validate($err:ty) $validate:expr;)?
        $(plugins: [$($plugin:path),+ $(,)?];)?
    } => {
        $crate::define_custom! {
            #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            $(#[$meta])*
            $vis $wrapper: String;
            $(adjust $adjust;)?
            $(ensure $ensure;)?
            $(validate($err) $validate;)?
            $(plugins: [$($plugin),+ $(,)?];)?
        }
    };
}

define_string! {
    pub Username: String;
    adjust Trim;
    ensure Empty;
}
