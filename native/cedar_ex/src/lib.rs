use rustler;

mod atoms;
mod core;
mod ctx;

rustler::init!("Elixir.CedarPolicy.Native", load = ctx::on_load);
