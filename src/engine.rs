use serde::Deserialize;
use winreg::{RegKey, enums::*};
use winapi::um::winnt::HANDLE;
use winapi::shared::minwindef::FALSE;
use std::io;

#[derive(Deserialize, Clone)]
pub struct Db {
    pub version: String,
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
    pub artifacts: Vec<Artifact>,
}

#[derive(Deserialize, Clone)]
pub struct Artifact {
    #[serde(rename = "type")]
    pub kind: Kind,
    pub path: String,
}

#[derive(Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    RegistryKey,
    Mutex,
    NamedPipe,
}

/// An owned Windows HANDLE that calls CloseHandle on drop.
/// Mutexes and named pipes stay live as long as this is held.
pub struct Handle(HANDLE);
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { winapi::um::handleapi::CloseHandle(self.0); }
    }
}

pub fn load() -> Db {
    let json = crate::update::load_or_fetch()
        .unwrap_or_else(|| include_str!("../artifacts.json").to_string());
    serde_json::from_str(&json).expect("invalid artifacts.json")
}

/// Returns (applied, failed, live handles that must be kept alive by the caller).
pub fn apply(cat: &Category) -> (usize, usize, Vec<Handle>) {
    let (mut ok, mut fail) = (0, 0);
    let mut handles = Vec::new();

    for a in &cat.artifacts {
        match a.kind {
            Kind::RegistryKey => match reg_create(&a.path) {
                Ok(_) => ok += 1,
                Err(_) => fail += 1,
            },
            Kind::Mutex => match create_mutex(&a.path) {
                Ok(h) => { ok += 1; handles.push(Handle(h)); }
                Err(_) => fail += 1,
            },
            Kind::NamedPipe => match create_pipe(&a.path) {
                Ok(h) => { ok += 1; handles.push(Handle(h)); }
                Err(_) => fail += 1,
            },
        }
    }

    (ok, fail, handles)
}

/// Cleans up registry key artifacts. Mutex/pipe handles are closed by dropping the Vec<Handle>.
pub fn remove_registry(cat: &Category) -> usize {
    cat.artifacts.iter().rev()
        .filter(|a| a.kind == Kind::RegistryKey && reg_delete(&a.path).is_ok())
        .count()
}

// ── Registry ──────────────────────────────────────────────────────────────────

fn root_and_subkey(path: &str) -> io::Result<(RegKey, &str)> {
    if let Some(k) = path.strip_prefix("HKLM\\") {
        Ok((RegKey::predef(HKEY_LOCAL_MACHINE), k))
    } else if let Some(k) = path.strip_prefix("HKCU\\") {
        Ok((RegKey::predef(HKEY_CURRENT_USER), k))
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "unknown hive"))
    }
}

fn reg_create(path: &str) -> io::Result<()> {
    let (root, key) = root_and_subkey(path)?;
    root.create_subkey(key).map(|_| ())
}

fn reg_delete(path: &str) -> io::Result<()> {
    let (root, key) = root_and_subkey(path)?;
    root.delete_subkey_all(key)
}

// ── Mutex / NamedPipe ─────────────────────────────────────────────────────────

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn create_mutex(name: &str) -> io::Result<HANDLE> {
    let wide = to_wide(name);
    let h = unsafe {
        winapi::um::synchapi::CreateMutexW(std::ptr::null_mut(), FALSE, wide.as_ptr())
    };
    if h.is_null() { Err(io::Error::last_os_error()) } else { Ok(h) }
}

fn create_pipe(name: &str) -> io::Result<HANDLE> {
    let wide = to_wide(name);
    // ponytail: hardcoded PIPE_ACCESS_DUPLEX (0x3) and PIPE_TYPE_BYTE|PIPE_WAIT (0x0) constants
    let h = unsafe {
        winapi::um::namedpipeapi::CreateNamedPipeW(
            wide.as_ptr(),
            0x00000003, // PIPE_ACCESS_DUPLEX
            0x00000000, // PIPE_TYPE_BYTE | PIPE_WAIT
            1, 0, 0, 0,
            std::ptr::null_mut(),
        )
    };
    if h == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        Err(io::Error::last_os_error())
    } else {
        Ok(h)
    }
}
