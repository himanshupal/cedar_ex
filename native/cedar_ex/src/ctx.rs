use cedar_policy::Policy;
use rustler::{Encoder, Env, Error, NifResult, Resource, ResourceArc, Term, nif};
use std::sync::Mutex;

use crate::atoms;

pub struct Context {
    pub policy: Mutex<Policy>,
}

impl Resource for Context {
    const IMPLEMENTS_DESTRUCTOR: bool = true;

    fn destructor(self, _: Env<'_>) {
        println!("Dropping context resource... {:?}", atoms::ok())
    }
}

pub fn on_load(env: Env, _term: Term) -> bool {
    env.register::<Context>().is_ok()
}

#[nif]
fn new_context(p: &str) -> NifResult<ResourceArc<Context>> {
    match Policy::parse(None, p) {
        Ok(policy) => Ok(ResourceArc::new(Context {
            policy: Mutex::new(policy),
        })),
        Err(e) => {
            println!("{:?}", e);
            Err(Error::Atom("error"))
        }
    }
}

#[nif]
fn get_policy_as_json(ctx: ResourceArc<Context>) -> impl Encoder {
    let policy = ctx.policy.lock().unwrap();
    policy.to_json().unwrap().to_string()
}
