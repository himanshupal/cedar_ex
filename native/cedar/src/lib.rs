use rustler::{nif, Env, Resource, ResourceArc, Term};
use std::sync::Mutex;

mod atoms {
    rustler::atoms! {
        ok
    }
}

struct Context {
    counter: Mutex<i64>,
}

impl Resource for Context {
    const IMPLEMENTS_DESTRUCTOR: bool = true;

    fn destructor(self, _: Env<'_>) {
        let mut counter = self.counter.lock().unwrap();
        *counter = 0;

        println!("Dropping context resource... {:?}", atoms::ok())
    }
}

fn on_load(env: Env, _term: Term) -> bool {
    env.register::<Context>().is_ok()
}

#[nif]
fn new_context() -> ResourceArc<Context> {
    ResourceArc::new(Context {
        counter: Mutex::new(0),
    })
}

#[nif]
fn increment(ctx: ResourceArc<Context>) -> i64 {
    let mut counter = ctx.counter.lock().unwrap();
    *counter += 1;
    *counter
}

rustler::init!("Elixir.CedarPolicy.Native", load = on_load);
