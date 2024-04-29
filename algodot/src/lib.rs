use godot::prelude::*;
use tokio::{
    runtime::{Builder, Runtime},
    task::LocalSet,
};

mod algod;
// https://godot-rust.github.io/book/recipes/async-tokio.html

thread_local! {
    static EXECUTOR: &'static SharedLocalPool = {
        Box::leak(Box::default())
    };
}

#[derive(Default)]
struct SharedLocalPool {
    local_set: LocalSet,
}

impl futures::task::LocalSpawn for SharedLocalPool {
    fn spawn_local_obj(
        &self,
        future: futures::task::LocalFutureObj<'static, ()>,
    ) -> Result<(), futures::task::SpawnError> {
        self.local_set.spawn_local(future);

        Ok(())
    }
}

/* Using Async Cookbook Recipie to handle Async Tasks using Threads*/

#[derive(GodotClass)]
#[class(base=Node)]
struct AsyncExecutorDriver {
    runtime: Runtime,
}

impl AsyncExecutorDriver {
    fn new(_base: &Node) -> Self {
        AsyncExecutorDriver {
            runtime: Builder::new_current_thread()
                .enable_io() // optional, depending on your needs
                .enable_time() // optional, depending on your needs
                .build()
                .unwrap(),
        }
    }
}

//#[func]//[async] //[func] //#[methods]
impl AsyncExecutorDriver {
    //#[func]
    fn _process(&self, _delta: f64) {
        EXECUTOR.with(|e| {
            self.runtime
                .block_on(async {
                    e.local_set
                        .run_until(async { tokio::task::spawn_local(async {}).await })
                        .await
                })
                .unwrap()
        })
    }
}

// Rewrite Macros using gdext
//#[gdextension]
//#[godot_api]

//fn init(handle: InitHandle) {
//gdnative::tasks::register_runtime(&handle);
//gdnative::tasks::set_executor(EXECUTOR.with(|e| *e));
//    handle.add_class::<algod::Algodot>();
//    handle.add_class::<AsyncExecutorDriver>();
//}

/* godot_init!(init); */
