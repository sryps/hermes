#[macro_export]
macro_rules! arg_vars {
    (
        @method( $( $method:tt )* )

        @args(
            &self
            $( , $rest:tt )*
        )

        @out()
    ) => {
        $crate::arg_vars!(
            @method( $( $method )* )
            @args( $( $rest )* )
            @out( &self, )
        )
    };
    (
        @method( $( $method:tt )* )

        @args(
            $arg_var:ident : $arg_type:ty
            $( , $rest:tt )*
        )

        @out(
            $( $out:tt )*
        )
    ) => {
        $crate::arg_vars!(
            @method( $( $method )* )
            @args( $( $rest )* )
            @out( $( $out )* $arg_var, )
        )
    };
    (
        @method( $( $method:tt )* )

        @args(
            $(,)?
        )
        @out(
            $( $out:tt )*
        )
    ) => {
        $( $method )* ( $( $out )* )
    };
}
