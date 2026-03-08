use rustler::{Atom, NifStruct};

#[derive(NifStruct, Debug)]
#[module = "CedarPolicy.Error"]
pub(crate) struct ExError {
    pub(crate) reason: String,
    pub(crate) source: Atom,
}
