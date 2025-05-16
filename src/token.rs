use std::path::PathBuf;

use crate::pass::{Gopass, Libsecret, PassResult, PasswordManager};

pub(crate) enum Manager {
    Libsecret,
    Gopass { path: PathBuf },
}
pub(crate) fn set_token(
    mgr: &Manager,
    url: impl AsRef<str>,
    user: impl AsRef<str>,
    token: impl AsRef<str>,
) -> PassResult<()> {
    match mgr {
        Manager::Libsecret => Libsecret {}.set_token(url, user, token),
        Manager::Gopass { path } => Gopass {
            path: path.to_owned(),
        }
        .set_token(url, user, token),
    }
}
pub(crate) fn get_token(
    mgr: &Manager,
    url: impl AsRef<str>,
    user: impl AsRef<str>,
) -> PassResult<Vec<u8>> {
    match mgr {
        Manager::Libsecret => Libsecret {}.get_token(url, user),
        Manager::Gopass { path } => Gopass {
            path: path.to_owned(),
        }
        .get_token(url, user),
    }
}
