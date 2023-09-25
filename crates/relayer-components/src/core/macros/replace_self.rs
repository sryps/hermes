// test
crate::replace_self! {
    app: App,
    trait Foo<Self> {
        fn foo(self, arr: &[Self]) -> ();
    }
}

#[macro_export]
macro_rules! replace_self {
    (   $val:ident : $type:ident,
        $( $rest:tt )*
    ) => {
        $crate::replace_self!{
            @with($val : $type)
            @out()
            @rest( $( $rest )* )
            @outer()
        }
    };
    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest(  )
        @outer(
            {
                @front( $( $front:tt )* )
                @back( $( $back:tt )* )
            }
            $( $outer:tt )*
        )
    ) => {
        $crate::replace_self!{
            @with($val : $type)
            @out( $( $front )* { $( $out )* } )
            @rest( $( $back )* )
            @outer( $( $outer )* )
        }
    };
    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest(  )
        @outer(
            (
                @front( $( $front:tt )* )
                @back( $( $back:tt )* )
            )
            $( $outer:tt )*
        )
    ) => {
        $crate::replace_self!{
            @with($val : $type)
            @out( $( $front )* ( $( $out )* ) )
            @rest( $( $back )* )
            @outer( $( $outer )* )
        }
    };
    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest(  )
        @outer(
            [
                @front( $( $front:tt )* )
                @back( $( $back:tt )* )
            ]
            $( $outer:tt )*
        )
    ) => {
        $crate::replace_self!{
            @with($val : $type)
            @out( $( $front )* [ $( $out )* ] )
            @rest( $( $back )* )
            @outer( $( $outer )* )
        }
    };

    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest( )
        @outer( )
    ) => {
       $( $out )*
    };

    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest( ( $( $inner:tt )* ) $( $rest:tt )* )
        @outer( $( $outer:tt )* )
    ) => {
        $crate::replace_self! {
            @with( $val : $type )
            @out( )
            @rest( $( $inner )* )
            @outer(
                (
                    @front( $( $out )* )
                    @back( $( $rest )* )
                )
                $( $outer )*
            )
        }
    };
    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest( { $( $inner:tt )* } $( $rest:tt )* )
        @outer( $( $outer:tt )* )
    ) => {
        $crate::replace_self! {
            @with( $val : $type )
            @out( )
            @rest( $( $inner )* )
            @outer(
                {
                    @front( $( $out )* )
                    @back( $( $rest )* )
                }
                $( $outer )*
            )
        }
    };
    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest( [ $( $inner:tt )* ] $( $rest:tt )* )
        @outer( $( $outer:tt )* )
    ) => {
        $crate::replace_self! {
            @with( $val : $type )
            @out( )
            @rest( $( $inner )* )
            @outer(
                [
                    @front( $( $out )* )
                    @back( $( $rest )* )
                ]
                $( $outer )*
            )
        }
    };


    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest( Self $( $rest:tt )* )
        @outer( $( $outer:tt )* )
    ) => {
        $crate::replace_self! {
            @with( $val : $type )
            @out( $( $out )* $type )
            @rest( $( $rest )* )
            @outer( $( $outer )* )
        }
    };
    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest( self $( $rest:tt )* )
        @outer( $( $outer:tt )* )
    ) => {
        $crate::replace_self! {
            @with( $val : $type )
            @out( $( $out )* $val : $type )
            @rest( $( $rest )* )
            @outer( $( $outer )* )
        }
    };
    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest( &self $( $rest:tt )* )
        @outer( $( $outer:tt )* )
    ) => {
        $crate::replace_self! {
            @with( $val : $type )
            @out( $( $out )* $val : &$type )
            @rest( $( $rest )* )
            @outer( $( $outer )* )
        }
    };
    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest( &mut self $( $rest:tt )* )
        @outer( $( $outer:tt )* )
    ) => {
        $crate::replace_self! {
            @with( $val : $type )
            @out( $( $out )* $val : &mut $type )
            @rest( $( $rest )* )
            @outer( $( $outer )* )
        }
    };

    (   @with( $val:ident : $type:ident )
        @out( $( $out:tt )* )
        @rest( $current:tt $( $rest:tt )* )
        @outer( $( $outer:tt )* )
    ) => {
        $crate::replace_self!(
            @with( $val : $type )
            @out(
                $( $out )*
                $current
            )
            @rest( $( $rest )* )
            @outer( $( $outer )* )
        );
    };
}
