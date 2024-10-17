pub use paste::paste;

/// Macro for simplifying AsyncMethod impl
/// This macro simplifies the process of defining asynchronous methods for a Godot class.
//It takes a list of function definitions and generates the boilerplate needed to register them as asynchronous methods for the Algodot class.
//How it works:

//    For each function (fn $fn), it generates a corresponding struct (__ $fn) used to represent the method.
//    The macro then implements AsyncMethod<Algodot> for this struct. In the implementation, the function's body (provided by the user in $block) is executed asynchronously using spawn.
//    spawner.spawn takes a closure, where the context ($ctx), the instance of Algodot ($this), and the function arguments ($args) are provided.
//    The block of code provided by the user is inserted into this closure, making it execute within the asynchronous task.
//    It also automatically registers the method using the builder.method() call in register_methods.

#[macro_export]
macro_rules! asyncmethods {
    ($algod:ident, $node:ident, $this:ident, $( fn $fn:ident($ctx:ident, $args:ident) $block:block) *) => {
        $crate::paste! {
            $(
                #[allow(non_camel_case_types)]
                struct [<__ $fn>];

                impl AsyncMethod<Algodot> for [<__ $fn>] {
                    fn spawn_with(&self, spawner: Spawner<'_, Algodot>) {
                        spawner.spawn(|$ctx, $this, mut $args| {
                            #[allow(unused_variables)]
                            let ($algod, $node) = $this.map(|algodot, node| {
                                (Rc::clone(&algodot.algod), node.claim())
                            }).unwrap();

                            $block

                        });
                    }
                }
            ) *

            fn register_methods(builder: &ClassBuilder<Algodot>) {
                $ (
                    builder.method(stringify!($fn), Async::new([<__ $fn >])).done();
                ) *
            }
        }
    };
}

/// Converts from `Result<T, E>` to `Option<T>`, printing the error to godot's stderr.
#[macro_export]
macro_rules! godot_unwrap {
    ($res:ident) => {
        match $res {
            Ok(ok) => Some(ok),
            Err(err) => {
                godot_error!("{:?}", err);
                None
            }
        }
    };
}
