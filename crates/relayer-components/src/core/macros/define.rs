#[macro_export]
macro_rules! define_component {
    (
        @meta(
            $( #[ $attributes:meta ] )*
        )

        @provider(
            $provider_trait:ident [ $context_var:ident : $context_type:ident ]
        )

        @consumer(
            $consumer_trait:ident $( < $( $generic_type:ident ),*  $(,)? > )?
                $( :  $( $self_constraint:tt )* )?
        )

        @constraints(
            $( $extra_constraint:tt )*
        )
        @methods(
            $(
                $( $async:ident )? fn $method_name:ident (
                    $( $args:tt )*
                ) $( -> $return:ty )?
                ;
            )*
        )
    ) => {
        $( #[ $attributes ] )*
        pub trait $consumer_trait < $( $( $generic_type ),* )? >
            : $( $( $self_constraint )* )?
        where
            $( $extra_constraint )*
        {
            $(
                $( $async )? fn $method_name (
                    $( $args )*
                ) $( -> $return )?
                ;
            )*
        }

        $crate::replace_self! {
            $context_var : $context_type,

            $( #[ $attributes ] )*
            pub trait $provider_trait < $context_type, $( $( $generic_type ),* )? >
            where
                $( $context_type : $( $self_constraint )* , )?
                $( $extra_constraint )*
            {
                $(
                    $( $async )? fn $method_name (
                        $( $args )*
                    ) $( -> $return )?
                    ;
                )*
            }
        }
    }
}

use crate::std_prelude::*;
use async_trait::async_trait;

crate::define_component! {
    @meta(
        #[async_trait]
    )

    @provider(
        ActionPerformer[ context: Context ]
    )

    @consumer(
        CanPerformAction<Foo>: 'static
    )

    @constraints(
        Foo: Send
    )

    @methods(
        async fn perform_action(&self) -> Result<(), ()>;
    )
}
