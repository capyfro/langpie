use std::path::PathBuf;

use keyring::Entry;
use toml::Table;

pub(crate) trait PasswordManager {
    fn set_token(
        &self,
        url: impl AsRef<str>,
        user: impl AsRef<str>,
        token: impl AsRef<str>,
    ) -> PassResult<()>;
    fn get_token(&self, url: impl AsRef<str>, user: impl AsRef<str>) -> PassResult<Vec<u8>>;
}

pub(crate) struct Libsecret;
pub(crate) struct Gopass {
    pub(crate) path: PathBuf,
}

impl PasswordManager for Libsecret {
    fn set_token(
        &self,
        url: impl AsRef<str>,
        user: impl AsRef<str>,
        token: impl AsRef<str>,
    ) -> PassResult<()> {
        let entry = Entry::new(url.as_ref(), user.as_ref())?;
        let token = token.as_ref();
        entry.set_secret(token.as_bytes())?;
        Ok(())
    }

    fn get_token(&self, url: impl AsRef<str>, user: impl AsRef<str>) -> PassResult<Vec<u8>> {
        let entry = Entry::new(url.as_ref(), user.as_ref())?;
        Ok(entry.get_secret()?)
    }
}

impl PasswordManager for Gopass {
    fn set_token(
        &self,
        url: impl AsRef<str>,
        user: impl AsRef<str>,
        token: impl AsRef<str>,
    ) -> PassResult<()> {
        let mut path = self.path.join("git").join(url.as_ref()).join(user.as_ref());
        path.set_extension("gpg");
        Ok(password_store::PasswordStore::insert(
            path.to_str().unwrap(),
            token.as_ref(),
        )?)
    }
    fn get_token(&self, url: impl AsRef<str>, user: impl AsRef<str>) -> PassResult<Vec<u8>> {
        let mut path = self.path.join("git").join(url.as_ref()).join(user.as_ref());
        path.set_extension("gpg");
        let (_, pass) = password_store::PasswordStore::get(path.to_str().unwrap())?;
        Ok(pass.into_bytes())
    }
}

pub(crate) fn gopass_conf() -> PathBuf {
    match std::env::home_dir() {
        Some(h) => match std::fs::read_to_string(h.join(".config").join("gopass").join("config")) {
            Ok(s) => match s.parse::<Table>() {
                Ok(t) => match t["mounts"].as_table() {
                    Some(m) => match m["path"].as_str() {
                        Some(p) => Some(PathBuf::from(p)),
                        None => None,
                    },
                    None => None,
                },
                Err(_) => None,
            },
            _ => None,
        }
        .unwrap_or(h.join(".local/share/gopass/stores/root")),
        None => panic!("No home directory"),
    }
}

#[derive(Debug)]
pub(crate) enum PassError {
    Libsecret(keyring::error::Error),
    Gopass(password_store::Error),
}

impl std::fmt::Display for PassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PassError::Libsecret(ref err) => write!(f, "Libsecret failure: {}", err),
            PassError::Gopass(ref err) => write!(f, "Gopass failure: {}", err),
        }
    }
}

impl std::error::Error for PassError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            PassError::Libsecret(ref err) => Some(err),
            PassError::Gopass(ref err) => Some(err),
        }
    }
}

impl From<keyring::error::Error> for PassError {
    fn from(value: keyring::error::Error) -> Self {
        PassError::Libsecret(value)
    }
}

impl From<password_store::Error> for PassError {
    fn from(value: password_store::Error) -> Self {
        PassError::Gopass(value)
    }
}

pub(crate) type PassResult<T> = Result<T, PassError>;
