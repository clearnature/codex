use super::ConfiguredGitMarketplace;
use codex_config::types::MarketplaceSourceType;
use codex_i18n::current;
use codex_i18n::tr_with;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use tempfile::TempDir;
use tracing::warn;

const MARKETPLACE_INSTALL_METADATA_FILE: &str = ".codex-marketplace-install.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
struct InstalledMarketplaceMetadata {
    source_type: MarketplaceSourceType,
    source: String,
    ref_name: Option<String>,
    sparse_paths: Vec<String>,
    revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InstalledMarketplaceSnapshot {
    marketplace_name: String,
    exists: bool,
    metadata: Option<InstalledMarketplaceMetadata>,
}

pub(super) fn read_installed_marketplace_snapshot(
    root: &Path,
    marketplace_name: &str,
) -> InstalledMarketplaceSnapshot {
    if !root.exists() {
        return InstalledMarketplaceSnapshot {
            marketplace_name: marketplace_name.to_string(),
            exists: false,
            metadata: None,
        };
    }
    let metadata = match std::fs::read_to_string(installed_marketplace_metadata_path(root)) {
        Ok(metadata) => metadata,
        Err(_) => {
            return InstalledMarketplaceSnapshot {
                marketplace_name: marketplace_name.to_string(),
                exists: true,
                metadata: None,
            };
        }
    };
    let metadata = match serde_json::from_str::<InstalledMarketplaceMetadata>(&metadata) {
        Ok(metadata) => Some(metadata),
        Err(err) => {
            warn!(
                marketplace = marketplace_name,
                error = %err,
                "failed to parse activated marketplace metadata"
            );
            None
        }
    };
    InstalledMarketplaceSnapshot {
        marketplace_name: marketplace_name.to_string(),
        exists: true,
        metadata,
    }
}

pub(super) fn installed_marketplace_metadata_matches(
    snapshot: &InstalledMarketplaceSnapshot,
    marketplace: &ConfiguredGitMarketplace,
    revision: &str,
) -> bool {
    snapshot.metadata.as_ref() == Some(&installed_marketplace_metadata(marketplace, revision))
}

pub(super) fn write_installed_marketplace_metadata(
    root: &Path,
    marketplace: &ConfiguredGitMarketplace,
    revision: &str,
) -> Result<(), String> {
    let metadata = installed_marketplace_metadata(marketplace, revision);
    let contents = serde_json::to_string_pretty(&metadata).map_err(|err| {
        tr_with(
            current(),
            "failed to serialize activated marketplace metadata: {0}",
            &[&err.to_string()],
        )
    })?;
    std::fs::write(installed_marketplace_metadata_path(root), contents).map_err(|err| {
        tr_with(
            current(),
            "failed to write activated marketplace metadata: {0}",
            &[&err.to_string()],
        )
    })
}

pub(super) fn activate_marketplace_root(
    destination: &Path,
    staged_dir: TempDir,
    previous_snapshot: &InstalledMarketplaceSnapshot,
    after_activate: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let staged_root = staged_dir.path();
    let Some(parent) = destination.parent() else {
        return Err(tr_with(
            current(),
            "failed to determine marketplace install parent for {0}",
            &[&destination.display().to_string()],
        ));
    };
    std::fs::create_dir_all(parent).map_err(|err| {
        tr_with(
            current(),
            "failed to create marketplace install parent {1}: {0}",
            &[&err.to_string(), &parent.display().to_string()],
        )
    })?;

    if destination.exists() {
        let backup_dir = tempfile::Builder::new()
            .prefix("marketplace-backup-")
            .tempdir_in(parent)
            .map_err(|err| {
                tr_with(
                    current(),
                    "failed to create marketplace backup directory in {1}: {0}",
                    &[&err.to_string(), &parent.display().to_string()],
                )
            })?;
        let backup_root = backup_dir.path().join("root");
        std::fs::rename(destination, &backup_root).map_err(|err| {
            tr_with(
                current(),
                "failed to move previous marketplace root out of the way at {1}: {0}",
                &[&err.to_string(), &destination.display().to_string()],
            )
        })?;

        if let Err(err) = std::fs::rename(staged_root, destination) {
            let rollback_result = std::fs::rename(&backup_root, destination);
            return match rollback_result {
                Ok(()) => Err(tr_with(
                    current(),
                    "failed to activate upgraded marketplace at {1}: {0}",
                    &[&err.to_string(), &destination.display().to_string()],
                )),
                Err(rollback_err) => {
                    let backup_path = backup_dir.keep().join("root");
                    Err(tr_with(
                        current(),
                        "failed to activate upgraded marketplace at {2}: {0}; failed to restore previous marketplace root (left at {3}): {1}",
                        &[
                            &err.to_string(),
                            &rollback_err.to_string(),
                            &destination.display().to_string(),
                            &backup_path.display().to_string(),
                        ],
                    ))
                }
            };
        }

        let activation_result = if read_installed_marketplace_snapshot(
            &backup_root,
            &previous_snapshot.marketplace_name,
        ) != *previous_snapshot
        {
            Err(installed_marketplace_snapshot_changed_error(
                previous_snapshot,
            ))
        } else {
            after_activate()
        };
        if let Err(err) = activation_result {
            let remove_result = std::fs::remove_dir_all(destination);
            let rollback_result =
                remove_result.and_then(|()| std::fs::rename(&backup_root, destination));
            return match rollback_result {
                Ok(()) => Err(err),
                Err(rollback_err) => {
                    let backup_path = backup_dir.keep().join("root");
                    Err(tr_with(
                        current(),
                        "{0}; failed to restore previous marketplace root at {2} (left at {3}): {1}",
                        &[
                            err.as_str(),
                            &rollback_err.to_string(),
                            &destination.display().to_string(),
                            &backup_path.display().to_string(),
                        ],
                    ))
                }
            };
        }

        return Ok(());
    }

    std::fs::rename(staged_root, destination).map_err(|err| {
        tr_with(
            current(),
            "failed to activate upgraded marketplace at {1}: {0}",
            &[&err.to_string(), &destination.display().to_string()],
        )
    })?;
    let activation_result = if previous_snapshot.exists {
        Err(installed_marketplace_snapshot_changed_error(
            previous_snapshot,
        ))
    } else {
        after_activate()
    };
    if let Err(err) = activation_result {
        let remove_result = std::fs::remove_dir_all(destination);
        return match remove_result {
            Ok(()) => Err(err),
            Err(remove_err) => Err(tr_with(
                current(),
                "{0}; failed to remove newly activated marketplace root at {2}: {1}",
                &[
                    err.as_str(),
                    &remove_err.to_string(),
                    &destination.display().to_string(),
                ],
            )),
        };
    }

    Ok(())
}

fn installed_marketplace_snapshot_changed_error(snapshot: &InstalledMarketplaceSnapshot) -> String {
    tr_with(
        current(),
        "installed marketplace `{0}` changed while auto-upgrade was in flight",
        &[snapshot.marketplace_name.as_str()],
    )
}

fn installed_marketplace_metadata(
    marketplace: &ConfiguredGitMarketplace,
    revision: &str,
) -> InstalledMarketplaceMetadata {
    InstalledMarketplaceMetadata {
        source_type: MarketplaceSourceType::Git,
        source: marketplace.source.clone(),
        ref_name: marketplace.ref_name.clone(),
        sparse_paths: marketplace.sparse_paths.clone(),
        revision: revision.to_string(),
    }
}

fn installed_marketplace_metadata_path(root: &Path) -> PathBuf {
    root.join(MARKETPLACE_INSTALL_METADATA_FILE)
}
