use cedar_policy;
use rustler::{NifStruct, nif};

#[derive(NifStruct)]
#[module = "Version"]
struct ExVersion {
    build: String,
    pre: String,
    patch: u64,
    minor: u64,
    major: u64,
}

impl From<semver::Version> for ExVersion {
    fn from(value: semver::Version) -> Self {
        Self {
            build: value.build.to_string(),
            pre: value.pre.to_string(),
            patch: value.patch,
            minor: value.minor,
            major: value.major,
        }
    }
}

#[nif]
fn get_lang_version() -> ExVersion {
    ExVersion::from(cedar_policy::get_lang_version())
}

#[nif]
fn get_sdk_version() -> ExVersion {
    ExVersion::from(cedar_policy::get_sdk_version())
}
