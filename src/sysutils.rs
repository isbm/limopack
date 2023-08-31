use std::io::{Error, ErrorKind};

#[link(name = "c")]
extern "C" {
    fn geteuid() -> u32;
    fn getegid() -> u32;
}

/// Returns true if the specified UID matches
fn is_uid(uid: i8) -> bool {
    unsafe { geteuid() == uid.try_into().unwrap() }
}

// Returns true if the specified GID matches
fn is_gid(gid: i8) -> bool {
    unsafe { getegid() == gid.try_into().unwrap() }
}

/// Returns no error if user is root
pub fn user_is_root() -> Result<(), Error> {
    if !is_uid(0) || !is_gid(0) {
        return Err(Error::new(ErrorKind::PermissionDenied, "User requires root privileges"));
    }
    Ok(())
}
