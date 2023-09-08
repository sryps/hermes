use crate::core::traits::sync::Async;

pub trait HasComponents: Async {
    type Components: Async;
}
pub trait DelegateComponent<Name>: Async {
    type Delegate;
}

#[macro_export]
macro_rules! delegate_component {
    ( $key:ty, $target:ident $( < $( $param:ident ),* $(,)? > )?, $forwarded:ty $(,)?  ) => {
        impl< $( $( $param ),* )* >
            $crate::core::traits::component::DelegateComponent<$key>
            for $target $( < $( $param ),* > )*
        where
            Self: $crate::core::traits::sync::Async,
        {
            type Delegate = $forwarded;
        }
    };
}

#[macro_export]
macro_rules! delegate_components {
    ( [$(,)?], $target:ident $( < $( $param:ident ),* $(,)? > )?, $forwarded:ty $(,)? ) => {

    };
    ( [$name:ty $(, $($rest:tt)* )?], $target:ident $( < $( $param:ident ),* $(,)? > )?, $forwarded:ty $(,)?  ) => {
        $crate::delegate_component!($name, $target $( < $( $param ),* > )*, $forwarded);
        $crate::delegate_components!([ $( $($rest)* )? ], $target $( < $( $param ),* > )*, $forwarded);
    };
}

#[macro_export]
macro_rules! callback {
    (  $root:ident :: $callback:ident ($($args:tt)*)) => {
        $root :: $callback!($($args)*);
    };
}

struct App;

crate::replace_self! {
    app: App,
    trait Foo {
        fn foo(self) -> ();
    }
}

#[macro_export]
macro_rules! replace_self {
    ($val:ident : $type:ident, $( $rest:tt )*) => {
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
        @rest( $current:tt $( $rest:tt )* )
        @outer( $( $outer:tt )* )
    ) => {
        $crate::replace_self!(
            @with( $val : $type )
            @out(
                $( $out )*
                $current
                // $crate::replace_self!( @with( $val: $type ), @out( ), @rest( $current ) );
            )
            @rest( $( $rest )* )
            @outer( $( $outer )* )
        );
    };
    // (   @with( $val:ident : $type:ident ),
    //     @out( $( $out:tt )* ),
    //     @rest( &self $( $rest:tt )* )
    // ) => {
    //     $crate::replace_self! {
    //         @with( $val : $type ),
    //         @out( $( $out )* $val : &$type ),
    //         @rest( $( $rest )* )
    //     }
    // };
    // (   @with( $val:ident : $type:ident ),
    //     @out( $( $out:tt )* ),
    //     @rest( &mut self $( $rest:tt )* )
    // ) => {
    //     $crate::replace_self!(
    //         @with( $val : $type ),
    //         @out( $( $out )* $val : &mut $type ),
    //         @rest( $( $rest )* )
    //     )
    // };
    // (   @with( $val:ident : $type:ident ),
    //     @out( $( $out:tt )* ),
    //     @rest( self $( $rest:tt )* )
    // ) => {
    //     $crate::replace_self!(
    //         @with( $val : $type ),
    //         @out( $( $out )* $val : $type ),
    //         @rest( $( $rest )* )
    //     )
    // };

    // (   @with( $val:ident : $type:ident ),
    //     @out( $( $out:tt )* ),
    //     @rest( ( $( $inner:tt )* ) $( $rest:tt )* )
    // ) => {
    //     $crate::replace_self! {
    //         @with( $val : $type ),
    //         @out(
    //             $( $out )* (
    //                 // $( $inner )*
    //                 $crate::replace_self! (
    //                     @with( $val : $type ),
    //                     @out(  ),
    //                     @rest( $( $inner )* )
    //                 )
    //             )
    //         ),
    //         @rest( $( $rest )* )
    //     }
    // };
    // (   @with( $val:ident : $type:ident ),
    //     @out( $( $out:tt )* ),
    //     @rest( < $( $inner:tt )* > $( $rest:tt )* )
    // ) => {
    //     $crate::replace_self!(
    //         @with( $val : $type ),
    //         @out(
    //             $out <
    //                 $crate::replace_self!(
    //                     @with( $val : $type ),
    //                     @out(  ),
    //                     @rest( $( $inner )* )
    //                 )
    //             >
    //         ),
    //         @rest( $( $rest )* )
    //     )
    // };
    // (   @with( $val:ident : $type:ident ),
    //     @out( $( $out:tt )* ),
    //     @rest( [ $( $rest:tt )* ] )
    // ) => {
    //     $crate::replace_self!(
    //         @with( $val : $type ),
    //         @out(
    //             $out [
    //                 $crate::replace_self!(
    //                     @with( $val : $type ),
    //                     @out(  ),
    //                     @rest( $( $inner )* )
    //                 )
    //             ]
    //         ),
    //         @rest( $( $rest )* )
    //     )
    // };
    // (   @with( $val:ident : $type:ident ),
    //     @out( $( $out:tt )* ),
    //     @rest( $current:tt )
    // ) => {
    //     $( $out )*
    //     $current
    //     // $crate::replace_self!( @with( $val: $type ), @out( ), @rest( $current ) )
    // };
    // (   @with( $val:ident : $type:ident ),
    //     @out( $( $out:tt )* ),
    //     $rest:tt
    // ) => {
    //     $rest
    // };
    // (   @with( $val:ident : $type:ident ),
    //     @out( $( $out:tt )* ),
    //     $( $rest:tt ),*
    // ) => {
    //     $( $crate::replace_self!( @with( $val : $type ), $rest ) ),             *
    // };
}
