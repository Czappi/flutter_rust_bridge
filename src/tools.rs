use std::{collections::HashMap, convert::TryFrom, path::PathBuf, str::FromStr};

use cargo_metadata::{Version, VersionReq};
use log::debug;
use serde::Deserialize;

use crate::{commands::call_shell, error::Error};

#[derive(Debug, PartialEq)]
pub(crate) enum DartToolchain {
    Dart,
    Flutter,
}

impl ToString for DartToolchain {
    fn to_string(&self) -> String {
        match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter",
        }
        .to_string()
    }
}

impl DartToolchain {
    #[inline]
    pub fn manifest_filename() -> &'static str {
        "pubspec.yaml"
    }
    #[inline]
    pub fn lock_filename() -> &'static str {
        "pubspec.lock"
    }
}

impl DartToolchain {
    pub(crate) fn as_run_command(&self) -> &'static str {
        match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter pub",
        }
    }
    pub(crate) fn available(&self) -> bool {
        match self {
            DartToolchain::Dart => call_shell("dart --version").status.success(),
            DartToolchain::Flutter => call_shell("flutter --version").status.success(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DartRepository {
    pub(crate) at: PathBuf,
    pub(crate) toolchain: DartToolchain,
}

impl FromStr for DartRepository {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("Guessing toolchain the runner is run into");
        let filename = DartToolchain::lock_filename();
        let lock_file = read_file(s, filename)?;
        let lock_file: PubspecLock = serde_yaml::from_str(&lock_file)
            .map_err(|_| anyhow::Error::msg(format!("unable to parse {} in {}", filename, s)))?;
        if lock_file
            .packages
            .contains_key(&DartToolchain::Flutter.to_string())
        {
            return Ok(DartRepository {
                at: PathBuf::from(s),
                toolchain: DartToolchain::Flutter,
            });
        }
        Ok(DartRepository {
            at: PathBuf::from(s),
            toolchain: DartToolchain::Dart,
        })
    }
}

impl DartRepository {
    pub(crate) fn toolchain_available(&self) -> bool {
        self.toolchain.available()
    }
    pub(crate) fn has_specified(
        &self,
        package: &str,
        manager: PackageManager,
        requirement: &VersionReq,
    ) -> anyhow::Result<()> {
        let at = self.at.to_str().unwrap();
        debug!("Checking presence of {} in {} at {}", package, manager, at);
        let manifest_file = read_file(at, DartToolchain::manifest_filename())?;
        let manifest_file: Pubspec = serde_yaml::from_str(&manifest_file).map_err(|_| {
            anyhow::Error::msg(format!(
                "unable to parse {} in {}",
                DartToolchain::manifest_filename(),
                at
            ))
        })?;
        let deps = if manager == PackageManager::DevDependencies {
            manifest_file.dev_dependencies.unwrap_or_default()
        } else {
            manifest_file.dependencies.unwrap_or_default()
        };
        let version = deps.get(package);
        let version = match version {
            Some(v) => PackageVersionKind::try_from(v).ok(),
            None => None,
        };
        match version {
            // if user specifies an exact version, let's see if it matches
            Some(PackageVersionKind::Exact(ref v)) if requirement.matches(v) => Ok(()),
            // if user specifies a range of versions, we cannot check if it matches yet, but let's not fail early
            Some(PackageVersionKind::Range(_)) => Ok(()),
            None => Err(anyhow::Error::new(Error::MissingDep {
                name: package.to_string(),
                manager,
                requirement: requirement.to_string(),
            })),
            Some(PackageVersionKind::Exact(_)) => Err(anyhow::Error::new(Error::InvalidDep {
                name: package.to_string(),
                manager,
                requirement: requirement.to_string(),
            })),
        }
    }
    pub(crate) fn has_installed(
        &self,
        package: &str,
        manager: PackageManager,
        requirement: &VersionReq,
    ) -> anyhow::Result<()> {
        let at = self.at.to_str().unwrap();
        debug!("Checking presence of {} in {} at {}", package, manager, at);
        let lock_file = read_file(at, DartToolchain::lock_filename())?;
        let lock_file: PubspecLock = serde_yaml::from_str(&lock_file).map_err(|_| {
            anyhow::Error::msg(format!(
                "unable to parse {} in {}",
                DartToolchain::lock_filename(),
                at
            ))
        })?;
        let dependency = lock_file.packages.get(package);
        let version = match dependency {
            Some(dependency) => {
                if (manager == PackageManager::Dependencies
                    && dependency.dependency != "direct main")
                    || (manager == PackageManager::DevDependencies
                        && dependency.dependency != "direct dev")
                {
                    return Err(anyhow::Error::new(Error::InvalidDep {
                        name: package.to_string(),
                        manager,
                        requirement: requirement.to_string(),
                    }));
                }
                PackageVersionKind::try_from(dependency).map_err(|_| {
                    anyhow::Error::msg(format!(
                        "unable to parse {} version in {}",
                        package,
                        DartToolchain::lock_filename(),
                    ))
                })?
            }
            None => {
                return Err(anyhow::Error::new(Error::MissingDep {
                    name: package.to_string(),
                    manager,
                    requirement: requirement.to_string(),
                }))
            }
        };

        match version {
            PackageVersionKind::Exact(ref v) if requirement.matches(v) => Ok(()),
            PackageVersionKind::Range(_) => Err(anyhow::Error::new(Error::StringError(format!(
                "unexpected version range for {} in {}",
                package,
                DartToolchain::lock_filename()
            )))),
            _ => Err(anyhow::Error::new(Error::InvalidDep {
                name: package.to_string(),
                manager,
                requirement: requirement.to_string(),
            })),
        }
    }
}

#[derive(Debug, Deserialize)]
struct PubspecLock {
    pub packages: HashMap<String, PubspecLockDependency>,
}

#[derive(Debug, Deserialize)]
struct PubspecLockDependency {
    pub dependency: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PackageVersion {
    Inline(String),
    Multiline { version: Option<String> },
}

impl PackageVersion {
    pub(crate) fn version(&self) -> Option<String> {
        match self {
            PackageVersion::Inline(v) => Some(v.clone()),
            PackageVersion::Multiline { version } => version.clone(),
        }
    }
}

#[derive(Debug)]
pub enum PackageVersionKind {
    Exact(Version),
    Range(VersionReq),
}

impl TryFrom<&PackageVersion> for PackageVersionKind {
    type Error = anyhow::Error;
    fn try_from(version: &PackageVersion) -> Result<Self, Self::Error> {
        if let Some(ref version) = version.version() {
            return Self::from_str(version);
        }
        Err(anyhow::anyhow!("no version found"))
    }
}

impl TryFrom<&PubspecLockDependency> for PackageVersionKind {
    type Error = anyhow::Error;
    fn try_from(dependency: &PubspecLockDependency) -> Result<Self, Self::Error> {
        Self::from_str(&dependency.version)
    }
}

impl FromStr for PackageVersionKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range: [char; 4] = ['>', '<', '=', '^'];
        if s.contains(range) {
            let version_req = VersionReq::parse(s)?;
            Ok(PackageVersionKind::Range(version_req))
        } else {
            let version = Version::parse(s)?;
            Ok(PackageVersionKind::Exact(version))
        }
    }
}

impl ToString for PackageVersionKind {
    fn to_string(&self) -> String {
        match self {
            PackageVersionKind::Exact(v) => v.to_string(),
            PackageVersionKind::Range(v) => v.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PackageManager {
    Dependencies,
    DevDependencies,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Dependencies => write!(f, "dependencies"),
            PackageManager::DevDependencies => write!(f, "dev_dependencies"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Pubspec {
    pub dependencies: Option<HashMap<String, PackageVersion>>,
    pub dev_dependencies: Option<HashMap<String, PackageVersion>>,
}

#[inline]
fn read_file(at: &str, filename: &str) -> anyhow::Result<String> {
    let file = PathBuf::from(at).join(filename);
    if !file.exists() {
        return Err(anyhow::Error::msg(format!(
            "missing {} in {}",
            filename, at
        )));
    }
    let content = std::fs::read_to_string(file)
        .map_err(|_| anyhow::Error::msg(format!("unable to read {} in {}", filename, at)))?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        str::FromStr,
    };

    use super::DartRepository;
    use super::DartToolchain;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref FRB_EXAMPLES_FOLDER: PathBuf = {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("frb_example")
        };
    }

    fn guess_toolchain_base(path: &Path, expect_toolchain: DartToolchain) {
        let repo = DartRepository::from_str(&path.to_string_lossy()).expect(&format!(
            "can get toolchain from {}",
            path.to_string_lossy()
        ));
        assert_eq!(repo.toolchain, expect_toolchain);
    }

    #[test]
    fn guess_dart_toolchain() {
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER.join("pure_dart").join("dart").as_path(),
            DartToolchain::Dart,
        );
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER
                .join("pure_dart_multi")
                .join("dart")
                .as_path(),
            DartToolchain::Dart,
        );
    }

    #[test]
    fn guess_flutter_toolchain() {
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER.join("with_flutter").as_path(),
            DartToolchain::Flutter,
        );
    }
}
