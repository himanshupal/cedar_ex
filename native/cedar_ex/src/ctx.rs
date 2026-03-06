use cedar_policy::{Context as CedarContext, EntityId, EntityTypeName, EntityUid, Policy, Request};
use rustler::{Encoder, Env, Error, NifResult, NifStruct, Resource, ResourceArc, Term, nif};
use std::{str::FromStr, sync::Mutex};

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

#[derive(NifStruct)]
#[module = "CedarPolicy.EntityUid"]
struct ExEntityUid {
    type_name: String,
    id: String,
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

#[nif]
fn create_request(p: ExEntityUid, a: ExEntityUid, r: ExEntityUid) -> impl Encoder {
    let p = EntityUid::from_type_name_and_id(
        EntityTypeName::from_str(p.type_name.as_str()).unwrap(),
        EntityId::from_str(p.id.as_str()).unwrap(),
    );
    let a = EntityUid::from_type_name_and_id(
        EntityTypeName::from_str(a.type_name.as_str()).unwrap(),
        EntityId::from_str(a.id.as_str()).unwrap(),
    );
    let r = EntityUid::from_type_name_and_id(
        EntityTypeName::from_str(r.type_name.as_str()).unwrap(),
        EntityId::from_str(r.id.as_str()).unwrap(),
    );

    Request::new(p, a, r, CedarContext::empty(), None).unwrap();
}
