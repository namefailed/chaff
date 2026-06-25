use serde::Deserialize;
use winreg::{RegKey, enums::*};
use winapi::um::winnt::HANDLE;
use winapi::shared::minwindef::FALSE;
use rand::seq::SliceRandom;
use std::io;
use std::path::PathBuf;

// ── Data model ────────────────────────────────────────────────────────────────

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
    /// If set, randomly sample this many process artifacts instead of spawning all.
    #[serde(default)]
    pub sample: Option<usize>,
    pub artifacts: Vec<Artifact>,
}

impl Category {
    pub fn has_processes(&self) -> bool {
        self.artifacts.iter().any(|a| a.kind == Kind::FakeProcess)
    }
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
    FakeProcess,
}

// ── RAII wrappers ─────────────────────────────────────────────────────────────

/// Owned Windows HANDLE — calls CloseHandle on drop.
pub struct Handle(HANDLE);
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { winapi::um::handleapi::CloseHandle(self.0); }
    }
}

/// Ghost process — a copy of our binary running under a fake process name.
/// Killed and waited on drop.
pub struct GhostProcess {
    pub name: String,
    child: std::process::Child,
}

impl Drop for GhostProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

// ── Apply result ──────────────────────────────────────────────────────────────

pub struct ApplyResult {
    pub ok: usize,
    pub fail: usize,
    pub reg_count: usize,
    pub handles: Vec<Handle>,
    pub ghosts: Vec<GhostProcess>,
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn load() -> Db {
    let json = crate::update::load_or_fetch()
        .unwrap_or_else(|| include_str!("../artifacts.json").to_string());
    serde_json::from_str(&json).expect("invalid artifacts.json")
}

/// Apply all artifacts in a category.
/// `process_limit` overrides the category's `sample` field for FakeProcess entries.
pub fn apply(cat: &Category, process_limit: Option<usize>) -> ApplyResult {
    let mut result = ApplyResult { ok: 0, fail: 0, reg_count: 0, handles: Vec::new(), ghosts: Vec::new() };

    let mut all: Vec<&Artifact> = cat.artifacts.iter().collect();

    // For process categories, shuffle and limit. Other categories apply everything.
    if cat.has_processes() {
        let limit = process_limit.or(cat.sample).unwrap_or(all.len()).min(all.len());
        all.shuffle(&mut rand::thread_rng());
        all.truncate(limit);
    }

    for a in all {
        match a.kind {
            Kind::RegistryKey => match reg_create(&a.path) {
                Ok(_) => { result.ok += 1; result.reg_count += 1; }
                Err(_) => result.fail += 1,
            },
            Kind::Mutex => match create_mutex(&a.path) {
                Ok(h) => { result.ok += 1; result.handles.push(Handle(h)); }
                Err(_) => result.fail += 1,
            },
            Kind::NamedPipe => match create_pipe(&a.path) {
                Ok(h) => { result.ok += 1; result.handles.push(Handle(h)); }
                Err(_) => result.fail += 1,
            },
            Kind::FakeProcess => match spawn_ghost(&a.path) {
                Ok(g) => { result.ok += 1; result.ghosts.push(g); }
                Err(_) => result.fail += 1,
            },
        }
    }

    result
}

/// Delete registry key artifacts for a category. Reverse order to clean children before parents.
pub fn remove_registry(cat: &Category) -> usize {
    cat.artifacts.iter().rev()
        .filter(|a| a.kind == Kind::RegistryKey && reg_delete(&a.path).is_ok())
        .count()
}

/// Remove the temp ghost binary directory after all GhostProcesses are dropped.
pub fn cleanup_ghost_temp() {
    let _ = std::fs::remove_dir_all(std::env::temp_dir().join("chaff"));
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
    // ponytail: 0x3 = PIPE_ACCESS_DUPLEX, 0x0 = PIPE_TYPE_BYTE|PIPE_WAIT
    let h = unsafe {
        winapi::um::namedpipeapi::CreateNamedPipeW(
            wide.as_ptr(),
            0x00000003,
            0x00000000,
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

// ── Ghost processes ───────────────────────────────────────────────────────────

fn ghost_binary_path(name: &str) -> io::Result<PathBuf> {
    let dir = std::env::temp_dir().join("chaff");
    std::fs::create_dir_all(&dir)?;
    let stem = name.trim_end_matches(".exe");
    Ok(dir.join(format!("{stem}.exe")))
}

fn spawn_ghost(name: &str) -> io::Result<GhostProcess> {
    use std::os::windows::process::CommandExt;

    let path = ghost_binary_path(name)?;

    // Only copy if the ghost binary doesn't already exist — can't overwrite a running .exe on Windows
    if !path.exists() {
        let our_exe = std::env::current_exe()?;
        std::fs::copy(&our_exe, &path)?;
    }

    let child = std::process::Command::new(&path)
        .arg("--ghost")
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn()?;

    Ok(GhostProcess { name: name.to_string(), child })
}
