use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Version {
    pub long: String,
    pub default: semver::Version,
    pub llvm: semver::Version,
}

impl Default for Version {
    fn default() -> Self {
        let default = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");
        let commit = env!("GIT_COMMIT_HASH");
        let mut llvm_major: u32 = 0;
        let mut llvm_minor: u32 = 0;
        let mut llvm_patch: u32 = 0;

        unsafe {
            llvm_sys::core::LLVMGetVersion(&mut llvm_major, &mut llvm_minor, &mut llvm_patch);
        }
        let llvm = semver::Version::new(llvm_major as u64, llvm_minor as u64, llvm_patch as u64);

        Self {
            long: format!("{default}+commit.{commit}.llvm-{llvm}"),
            default,
            llvm,
        }
    }
}
