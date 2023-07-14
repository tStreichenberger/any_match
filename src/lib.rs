#[macro_export]
macro_rules! any_match {
    (
        ($anyhow_error:ident):
        $catch_ident:ident => $catch_eval:block
    ) => {
        {
            let $catch_ident = $anyhow_error;
            $catch_eval
        }
    };
    (
        ($anyhow_error:ident):
        _ => $catch_eval:block
    ) => {
        {
            _ = $anyhow_error;
            $catch_eval
        }
    };
    (
        ($anyhow_error:ident):
        ($error_type:ty as $error_val:ident) => $eval:block
        $($muncher:tt)*
    ) => {
        match $anyhow_error.downcast::<$error_type>() {
            Ok($error_val) => $eval
            Err($anyhow_error) => $crate::any_match!(($anyhow_error): $($muncher)*)
        }
    }
}