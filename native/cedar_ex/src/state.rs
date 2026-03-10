use cedar_policy::{Authorizer, Entities, PolicySet};
use rustler::{Env, Resource, ResourceArc, Term, nif};
use std::sync::RwLock;

use crate::atoms;

pub(crate) struct State {
    pub(crate) entities: RwLock<Entities>,
    pub(crate) policy_set: RwLock<PolicySet>,
    pub(crate) authorizer: RwLock<Authorizer>,
}

impl Resource for State {
    const IMPLEMENTS_DESTRUCTOR: bool = false;

    fn destructor(self, _: Env<'_>) {
        println!("Dropping context resource... {:?}", atoms::ok())
    }
}

pub(crate) fn on_load(env: Env, _: Term) -> bool {
    env.register::<State>().is_ok()
}

#[nif]
pub(crate) fn new() -> ResourceArc<State> {
    ResourceArc::new(State {
        entities: RwLock::new(Entities::empty()),
        policy_set: RwLock::new(PolicySet::new()),
        authorizer: RwLock::new(Authorizer::new()),
    })
}
