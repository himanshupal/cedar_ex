use cedar_policy::{Entities, PolicySet};
use rustler::{Env, Resource, ResourceArc, Term, nif};
use std::sync::RwLock;

pub(crate) struct State {
    pub(crate) entities: RwLock<Entities>,
    pub(crate) policy_set: RwLock<PolicySet>,
}

impl Resource for State {
    const IMPLEMENTS_DESTRUCTOR: bool = false;
}

pub(crate) fn on_load(env: Env, _: Term) -> bool {
    env.register::<State>().is_ok()
}

#[nif]
pub(crate) fn new() -> ResourceArc<State> {
    ResourceArc::new(State {
        entities: RwLock::new(Entities::empty()),
        policy_set: RwLock::new(PolicySet::new()),
    })
}
