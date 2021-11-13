use std::fmt::Debug;
use anyhow::Result;
use std::path::Path;
use cargo::core::Shell;
use cargo_util::paths::copy as basecopy;


// TODO: allow (from, to) or (shell, from, to) both by macro?
// pub fn copy<P: AsRef<Path> + Debug, Q: AsRef<Path> + Debug>(shell: &mut Shell, from: P, to: Q) -> Result<u64> {
//     shell.verbose(
//         |s| s.note(format!("Copying {:?} -> {:?}", from, to)
//     ))?;
//     basecopy(from, to)
// }