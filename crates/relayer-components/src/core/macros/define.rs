#[macro_export]
macro_rules! define_component {
    (
        @name( $component_name:ident )

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

        $(
            @method(
                fn $method_name:ident (
                    $( $args:tt )*
                ) $( -> $( $return:tt )+ )?
            )
        )+
    ) => {
        pub struct $component_name;

        $( #[ $attributes ] )*
        pub trait $consumer_trait < $( $( $generic_type ),* )? >
            : $( $( $self_constraint )* )?
        where
            $( $extra_constraint )*
        {
            $(
                fn $method_name (
                    $( $args )*
                ) $( -> $( $return )* )?
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
                    fn $method_name (
                        $( $args )*
                    ) $( -> $( $return )* )?
                    ;
                )*
            }
        }

        // $( #[ $attributes ] )*
        // impl< $context_type, $( $( $generic_type ),* )? >
        //     $consumer_trait < $( $( $generic_type ),* )? >
        //     for $context_type
        // where
        //     $context_type: $crate::core::traits::component::HasComponents,
        //     $context_type :: Components : $provider_trait < $context_type, $( $( $generic_type ),* )? >,
        //     $( $context_type : $( $self_constraint )* , )?
        //     $( $extra_constraint )*
        // {
        //     $(
        //         fn $method_name (
        //             $( $args )*
        //         ) $( -> $( $return )* )?
        //         {
        //             $crate::arg_vars!(
        //                 @method( $context_type :: Components :: $method_name )
        //                 @args( $( $args )* )
        //                 @out( )
        //             )
        //         }
        //     )*
        // }

    }
}

use crate::core::traits::error::HasErrorType;
use crate::std_prelude::*;
use async_trait::async_trait;

crate::define_component! {
    @name( ActionPerformerComponent )

    @meta(
        #[async_trait]
    )

    @provider(
        ActionPerformer[ context: Context ]
    )

    @consumer(
        CanPerformAction<Foo>: HasErrorType
    )

    @constraints(
        Foo: Send
    )

    @method(
        fn perform_action(&self) -> Result<(), Self::Error>
    )
}
