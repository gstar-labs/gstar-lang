//! Helpers shared by the integration tests.

#![allow(
    dead_code,
    reason = "each test binary uses a different subset of these helpers"
)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A scratch directory under the system temp directory, deleted on drop.
pub(crate) struct Scratch(PathBuf);

impl Scratch {
    /// A fresh, empty scratch directory unique to this test and process.
    pub(crate) fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("gstar-xtask-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        Self(dir)
    }

    /// The scratch directory.
    pub(crate) fn path(&self) -> &Path {
        &self.0
    }

    /// Writes `contents` to `relative`, creating parent directories.
    pub(crate) fn write(&self, relative: &str, contents: &str) {
        let path = self.0.join(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create parent dir");
        }
        std::fs::write(path, contents).expect("write file");
    }

    /// Runs git here with a fixed identity and no signing, and asserts success.
    pub(crate) fn git(&self, args: &[&str]) {
        let out = Command::new("git")
            .args([
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .current_dir(&self.0)
            .output()
            .expect("run git");
        assert!(out.status.success(), "git {args:?}: {out:?}");
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Runs the real `xtask` binary with `args` in `dir`.
pub(crate) fn xtask_in(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run xtask")
}
