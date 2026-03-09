mod atoms;
mod common;
mod entity;
mod error;
mod policy;
mod schema;
mod state;
mod template;
mod version;

rustler::init!("Elixir.CedarPolicy.Native", load = state::on_load);
