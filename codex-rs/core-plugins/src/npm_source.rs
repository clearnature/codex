use crate::plugin_bundle_archive::unpack_plugin_bundle_tar_gz;
use codex_i18n::current;
use codex_i18n::tr_with;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

const NPM_PLUGIN_SOURCE_STAGING_DIR: &str = "plugins/.marketplace-plugin-source-staging";
const NPM_PLUGIN_SOURCE_MAX_ARCHIVE_BYTES: u64 = 50 * 1024 * 1024;
const NPM_PLUGIN_SOURCE_MAX_EXTRACTED_BYTES: u64 = 250 * 1024 * 1024;
const NPM_PACKAGE_ARCHIVE_ROOT: &str = "package";

pub(crate) fn materialize_npm_plugin_source(
    codex_home: &Path,
    package: &str,
    version: Option<&str>,
    registry: Option<&str>,
) -> Result<(AbsolutePathBuf, TempDir), String> {
    materialize_npm_plugin_source_with_command(
        codex_home,
        package,
        version,
        registry,
        OsStr::new(npm_command()),
    )
}

fn materialize_npm_plugin_source_with_command(
    codex_home: &Path,
    package: &str,
    version: Option<&str>,
    registry: Option<&str>,
    npm_command: &OsStr,
) -> Result<(AbsolutePathBuf, TempDir), String> {
    let staging_root = codex_home.join(NPM_PLUGIN_SOURCE_STAGING_DIR);
    fs::create_dir_all(&staging_root).map_err(|err| {
        tr_with(
            current(),
            "failed to create marketplace plugin source staging directory {1}: {0}",
            &[&err.to_string(), &staging_root.display().to_string()],
        )
    })?;
    let tempdir = tempfile::Builder::new()
        .prefix("marketplace-plugin-source-")
        .tempdir_in(&staging_root)
        .map_err(|err| {
            tr_with(
                current(),
                "failed to create marketplace plugin source staging directory in {1}: {0}",
                &[&err.to_string(), &staging_root.display().to_string()],
            )
        })?;

    pack_npm_package(tempdir.path(), package, version, registry, npm_command)?;
    let archive_path = find_npm_package_archive(tempdir.path())?;
    let archive_bytes = read_npm_package_archive(&archive_path)?;

    let extraction_root = tempdir.path().join("extracted");
    unpack_plugin_bundle_tar_gz(
        &archive_bytes,
        &extraction_root,
        NPM_PLUGIN_SOURCE_MAX_EXTRACTED_BYTES,
    )
    .map_err(|err| {
        tr_with(
            current(),
            "failed to extract npm plugin package: {0}",
            &[&err.to_string()],
        )
    })?;
    let plugin_root = extraction_root.join(NPM_PACKAGE_ARCHIVE_ROOT);
    if !plugin_root.is_dir() {
        return Err(tr_with(
            current(),
            "npm pack completed without creating plugin package directory {0}",
            &[&plugin_root.display().to_string()],
        ));
    }
    validate_npm_package_metadata(&plugin_root, package)?;
    let plugin_root = AbsolutePathBuf::try_from(plugin_root).map_err(|err| {
        tr_with(
            current(),
            "failed to resolve materialized plugin source path: {0}",
            &[&err.to_string()],
        )
    })?;
    Ok((plugin_root, tempdir))
}

fn pack_npm_package(
    destination: &Path,
    package: &str,
    version: Option<&str>,
    registry: Option<&str>,
    npm_command: &OsStr,
) -> Result<(), String> {
    let package_spec = version.map_or_else(
        || package.to_string(),
        |version| format!("{package}@{version}"),
    );
    let mut command = Command::new(npm_command);
    command
        .current_dir(destination)
        .arg("pack")
        .arg("--ignore-scripts")
        .arg("--pack-destination")
        .arg(destination);
    if let Some(registry) = registry {
        command.arg("--registry").arg(registry);
    }
    command.arg("--").arg(package_spec);

    let output = command.output().map_err(|err| {
        tr_with(
            current(),
            "failed to run npm pack: {0}",
            &[&err.to_string()],
        )
    })?;
    if output.status.success() {
        return Ok(());
    }

    Err(tr_with(
        current(),
        "npm pack failed with status {0}\nstdout:\n{1}\nstderr:\n{2}",
        &[
            &output.status.to_string(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim(),
        ],
    ))
}

fn find_npm_package_archive(destination: &Path) -> Result<PathBuf, String> {
    let mut archives = fs::read_dir(destination)
        .map_err(|err| {
            tr_with(
                current(),
                "failed to read npm pack destination: {0}",
                &[&err.to_string()],
            )
        })?
        .filter_map(std::result::Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            let is_file = entry.file_type().is_ok_and(|file_type| file_type.is_file());
            (is_file && path.extension() == Some(OsStr::new("tgz"))).then_some(path)
        })
        .collect::<Vec<_>>();
    if archives.len() != 1 {
        return Err(tr_with(
            current(),
            "npm pack completed with {0} package archives; expected exactly one",
            &[&archives.len().to_string()],
        ));
    }
    Ok(archives.remove(0))
}

fn read_npm_package_archive(archive_path: &Path) -> Result<Vec<u8>, String> {
    let archive_size = fs::metadata(archive_path)
        .map_err(|err| {
            tr_with(
                current(),
                "failed to inspect npm package archive: {0}",
                &[&err.to_string()],
            )
        })?
        .len();
    if archive_size > NPM_PLUGIN_SOURCE_MAX_ARCHIVE_BYTES {
        return Err(tr_with(
            current(),
            "npm package archive is {0} bytes, exceeding maximum size of {1} bytes",
            &[
                &archive_size.to_string(),
                &NPM_PLUGIN_SOURCE_MAX_ARCHIVE_BYTES.to_string(),
            ],
        ));
    }
    fs::read(archive_path).map_err(|err| {
        tr_with(
            current(),
            "failed to read npm package archive: {0}",
            &[&err.to_string()],
        )
    })
}

fn validate_npm_package_metadata(plugin_root: &Path, package: &str) -> Result<(), String> {
    #[derive(Deserialize)]
    struct NpmPackageMetadata {
        name: String,
    }

    let package_json_path = plugin_root.join("package.json");
    let package_json = fs::read_to_string(&package_json_path).map_err(|err| {
        tr_with(
            current(),
            "failed to read npm plugin package metadata {1}: {0}",
            &[&err.to_string(), &package_json_path.display().to_string()],
        )
    })?;
    let metadata: NpmPackageMetadata = serde_json::from_str(&package_json).map_err(|err| {
        tr_with(
            current(),
            "failed to parse npm plugin package metadata {1}: {0}",
            &[&err.to_string(), &package_json_path.display().to_string()],
        )
    })?;
    if metadata.name != package {
        return Err(tr_with(
            current(),
            "npm plugin package name '{1}' does not match requested package '{0}'",
            &[package, metadata.name.as_str()],
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn npm_command() -> &'static str {
    "npm.cmd"
}

#[cfg(not(windows))]
fn npm_command() -> &'static str {
    "npm"
}

#[cfg(all(test, unix))]
#[path = "npm_source_tests.rs"]
mod tests;
