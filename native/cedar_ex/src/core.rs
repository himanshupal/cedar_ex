use cedar_policy;
use rustler::{NifStruct, nif};

#[derive(NifStruct)]
#[module = "CedarPolicy.Native.Version"]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

impl Version {
    fn from(ver: semver::Version) -> Self {
        Self {
            major: ver.major,
            minor: ver.minor,
            patch: ver.patch,
        }
    }
}

#[nif]
fn get_lang_version() -> Version {
    Version::from(cedar_policy::get_lang_version())
}

#[nif]
fn get_sdk_version() -> Version {
    Version::from(cedar_policy::get_sdk_version())
}
