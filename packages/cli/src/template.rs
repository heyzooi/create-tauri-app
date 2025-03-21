// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{fmt::Display, fs, io::Write, path, str::FromStr};

use anyhow::Context;
use rust_embed::RustEmbed;

use crate::{colors::*, manifest::Manifest, package_manager::PackageManager};

#[derive(RustEmbed)]
#[folder = "fragments"]
#[allow(clippy::upper_case_acronyms)]
struct FRAGMENTS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Template {
    Vanilla,
    VanillaTs,
    Vue,
    VueTs,
    Svelte,
    SvelteTs,
    React,
    ReactTs,
    Solid,
    SolidTs,
    Yew,
    Leptos,
    Sycamore,
    Angular,
    Preact,
    PreactTs,
}

impl Default for Template {
    fn default() -> Self {
        Template::Vanilla
    }
}

impl Template {
    pub const fn select_text<'a>(&self) -> &'a str {
        match self {
            Template::Vanilla => "Vanilla",
            Template::Vue => "Vue - (https://vuejs.org)",
            Template::Svelte => "Svelte - (https://svelte.dev/)",
            Template::React => "React - (https://reactjs.org/)",
            Template::Solid => "Solid - (https://www.solidjs.com/)",
            Template::Yew => "Yew - (https://yew.rs/)",
            Template::Leptos => "Leptos - (https://github.com/leptos-rs/leptos)",
            Template::Sycamore => "Sycamore - (https://sycamore-rs.netlify.app/)",
            Template::Angular => "Angular - (https://angular.io/)",
            Template::Preact => "Preact - (https://preactjs.com/)",
            _ => unreachable!(),
        }
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Template::Vanilla => write!(f, "vanilla"),
            Template::VanillaTs => write!(f, "vanilla-ts"),
            Template::Vue => write!(f, "vue"),
            Template::VueTs => write!(f, "vue-ts"),
            Template::Svelte => write!(f, "svelte"),
            Template::SvelteTs => write!(f, "svelte-ts"),
            Template::React => write!(f, "react"),
            Template::ReactTs => write!(f, "react-ts"),
            Template::Solid => write!(f, "solid"),
            Template::SolidTs => write!(f, "solid-ts"),
            Template::Yew => write!(f, "yew"),
            Template::Leptos => write!(f, "leptos"),
            Template::Sycamore => write!(f, "sycamore"),
            Template::Angular => write!(f, "angular"),
            Template::Preact => write!(f, "preact"),
            Template::PreactTs => write!(f, "preact-ts"),
        }
    }
}

impl FromStr for Template {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vanilla" => Ok(Template::Vanilla),
            "vanilla-ts" => Ok(Template::VanillaTs),
            "vue" => Ok(Template::Vue),
            "vue-ts" => Ok(Template::VueTs),
            "svelte" => Ok(Template::Svelte),
            "svelte-ts" => Ok(Template::SvelteTs),
            "react" => Ok(Template::React),
            "react-ts" => Ok(Template::ReactTs),
            "solid" => Ok(Template::Solid),
            "solid-ts" => Ok(Template::SolidTs),
            "yew" => Ok(Template::Yew),
            "leptos" => Ok(Template::Leptos),
            "sycamore" => Ok(Template::Sycamore),
            "angular" => Ok(Template::Angular),
            "preact" => Ok(Template::Preact),
            "preact-ts" => Ok(Template::PreactTs),
            _ => Err(format!(
                "{YELLOW}{s}{RESET} is not a valid template. Valid templates are [{}]",
                Template::ALL
                    .iter()
                    .map(|e| format!("{GREEN}{e}{RESET}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
        }
    }
}

impl<'a> Template {
    pub const ALL: &'a [Template] = &[
        Template::Vanilla,
        Template::VanillaTs,
        Template::Vue,
        Template::VueTs,
        Template::Svelte,
        Template::SvelteTs,
        Template::React,
        Template::ReactTs,
        Template::Solid,
        Template::SolidTs,
        Template::Yew,
        Template::Leptos,
        Template::Sycamore,
        Template::Angular,
        Template::Preact,
        Template::PreactTs,
    ];

    pub fn flavors<'b>(&self, pkg_manager: PackageManager) -> Option<&'b [Flavor]> {
        match self {
            Template::Vanilla => {
                if pkg_manager == PackageManager::Cargo {
                    None
                } else {
                    Some(&[Flavor::TypeScript, Flavor::JavaScript])
                }
            }
            Template::Vue => Some(&[Flavor::TypeScript, Flavor::JavaScript]),
            Template::Svelte => Some(&[Flavor::TypeScript, Flavor::JavaScript]),
            Template::React => Some(&[Flavor::TypeScript, Flavor::JavaScript]),
            Template::Solid => Some(&[Flavor::TypeScript, Flavor::JavaScript]),
            Template::Preact => Some(&[Flavor::TypeScript, Flavor::JavaScript]),
            _ => None,
        }
    }

    pub fn from_flavor(&self, flavor: Flavor) -> Self {
        match (self, flavor) {
            (Template::Vanilla, Flavor::TypeScript) => Template::VanillaTs,
            (Template::Vue, Flavor::TypeScript) => Template::VueTs,
            (Template::Svelte, Flavor::TypeScript) => Template::SvelteTs,
            (Template::React, Flavor::TypeScript) => Template::ReactTs,
            (Template::Solid, Flavor::TypeScript) => Template::SolidTs,
            (Template::Preact, Flavor::TypeScript) => Template::PreactTs,
            _ => *self,
        }
    }

    pub fn without_flavor(&self) -> Self {
        match self {
            Template::VanillaTs => Template::Vanilla,
            Template::VueTs => Template::Vue,
            Template::SvelteTs => Template::Svelte,
            Template::ReactTs => Template::React,
            Template::SolidTs => Template::Solid,
            Template::PreactTs => Template::Preact,
            _ => *self,
        }
    }

    pub const fn possible_package_managers(&self) -> &[PackageManager] {
        match self {
            Template::Vanilla => &[
                PackageManager::Cargo,
                PackageManager::Pnpm,
                PackageManager::Yarn,
                PackageManager::Npm,
                PackageManager::Bun,
            ],
            Template::VanillaTs => PackageManager::NODE,
            Template::Vue => PackageManager::NODE,
            Template::VueTs => PackageManager::NODE,
            Template::Svelte => PackageManager::NODE,
            Template::SvelteTs => PackageManager::NODE,
            Template::React => PackageManager::NODE,
            Template::ReactTs => PackageManager::NODE,
            Template::Solid => PackageManager::NODE,
            Template::SolidTs => PackageManager::NODE,
            Template::Yew => &[PackageManager::Cargo],
            Template::Leptos => &[PackageManager::Cargo],
            Template::Sycamore => &[PackageManager::Cargo],
            Template::Angular => PackageManager::NODE,
            Template::Preact => PackageManager::NODE,
            Template::PreactTs => PackageManager::NODE,
        }
    }

    pub const fn needs_trunk(&self) -> bool {
        matches!(self, Template::Sycamore | Template::Yew | Template::Leptos)
    }

    pub const fn needs_tauri_cli(&self) -> bool {
        matches!(
            self,
            Template::Sycamore | Template::Yew | Template::Leptos | Template::Vanilla
        )
    }

    pub const fn needs_wasm32_target(&self) -> bool {
        matches!(self, Template::Sycamore | Template::Yew | Template::Leptos)
    }

    pub fn render(
        &self,
        target_dir: &path::Path,
        pkg_manager: PackageManager,
        package_name: &str,
        alpha: bool,
        mobile: bool,
    ) -> anyhow::Result<()> {
        let manifest_bytes = FRAGMENTS::get(&format!("fragment-{self}/_cta_manifest_"))
            .with_context(|| "Failed to get manifest bytes")?
            .data;
        let manifest_str = String::from_utf8(manifest_bytes.to_vec())?;
        let manifest = Manifest::parse(&manifest_str, mobile)?;

        let lib_name = format!("{}_lib", package_name.replace('-', "_"));

        let write_file = |file: &str| -> anyhow::Result<()> {
            let manifest = manifest.clone();

            // remove the first component, which is certainly the fragment directory they were in before getting embeded into the binary
            let p = path::PathBuf::from(file)
                .components()
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .collect::<path::PathBuf>();

            let p = target_dir.join(p);
            let file_name = p.file_name().unwrap().to_string_lossy();

            let target_file_name = match &*file_name {
                "_gitignore" => ".gitignore",
                "_Cargo.toml" => "Cargo.toml",
                "_cta_manifest_" => return Ok(()),
                // conditional files:
                // are files that start with a special syntax
                //          "%(<list of flags separated by `-`>%)<file_name>"
                // flags are supported package managers, stable, alpha and mobile.
                // example: "%(pnpm-npm-yarn-stable-alpha)%package.json"
                name if name.starts_with("%(") && name[1..].contains(")%") => {
                    let mut s = name.strip_prefix("%(").unwrap().split(")%");
                    let (mut flags, name) = (
                        s.next().unwrap().split('-').collect::<Vec<_>>(),
                        s.next().unwrap(),
                    );

                    let for_stable = flags.contains(&"stable");
                    let for_alpha = flags.contains(&"alpha");
                    let for_mobile = flags.contains(&"mobile");

                    // remove these flags to only keep package managers flags
                    flags.retain(|e| !["stable", "alpha", "mobile"].contains(e));

                    if ((for_stable && !alpha)
                        || (for_alpha && alpha && !mobile)
                        || (for_mobile && alpha && mobile)
                        || (!for_stable && !for_alpha && !for_mobile))
                        && (flags.contains(&pkg_manager.to_string().as_str()) || flags.is_empty())
                    {
                        name
                    } else {
                        // skip writing this file
                        return Ok(());
                    }
                }
                _ => &file_name,
            };

            let mut data = FRAGMENTS::get(file).unwrap().data.to_vec();

            // Only modify specific set of files
            if [
                "Cargo.toml",
                "package.json",
                "tauri.conf.json",
                "main.rs",
                "vite.config.ts",
                "vite.config.js",
                "Trunk.toml",
                "angular.json",
            ]
            .contains(&target_file_name)
            {
                if let Ok(content) = String::from_utf8(data.to_vec()) {
                    // Replacement order is important
                    data = Self::replace_vars(
                        &content,
                        &lib_name,
                        package_name,
                        pkg_manager,
                        manifest,
                    )
                    .as_bytes()
                    .to_vec();
                }
            }

            let parent = p.parent().unwrap();
            fs::create_dir_all(parent)?;
            fs::write(parent.join(target_file_name), &data)?;
            Ok(())
        };

        for file in FRAGMENTS::iter().filter(|e| {
            path::PathBuf::from(e.to_string())
                .components()
                .next()
                .unwrap()
                .as_os_str()
                == "_base_"
        }) {
            write_file(&file)?;
        }

        // then write template files which can override files from base
        for file in FRAGMENTS::iter().filter(|e| {
            path::PathBuf::from(e.to_string())
                .components()
                .next()
                .unwrap()
                .as_os_str()
                == path::PathBuf::from(format!("fragment-{self}"))
        }) {
            write_file(&file)?;
        }

        // then write extra files specified in the fragment manifest
        for (src, dest) in manifest.files {
            let data = FRAGMENTS::get(&format!("_assets_/{src}"))
                .with_context(|| format!("Failed to get asset file bytes: {src}"))?
                .data;
            let dest = target_dir.join(dest);
            let parent = dest.parent().unwrap();
            fs::create_dir_all(parent)?;
            let mut file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(dest)?;
            file.write_all(&data)?;
        }

        Ok(())
    }

    fn replace_vars(
        content: &str,
        lib_name: &str,
        package_name: &str,
        pkg_manager: PackageManager,
        manifest: Manifest,
    ) -> String {
        manifest
            .replace_vars(content)
            .replace("~lib_name~", lib_name)
            .replace("~package_name~", package_name)
            .replace("~pkg_manager_run_command~", pkg_manager.run_cmd())
            .replace(
                "~double-dash~",
                if pkg_manager == PackageManager::Npm {
                    " --"
                } else {
                    ""
                },
            )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Flavor {
    JavaScript,
    TypeScript,
}

impl Display for Flavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flavor::JavaScript => write!(f, "JavaScript"),
            Flavor::TypeScript => write!(f, "TypeScript"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let manifest_file = r#"
        # Copyright 2019-2022 Tauri Programme within The Commons Conservancy
        # SPDX-License-Identifier: Apache-2.0
        # SPDX-License-Identifier: MIT

        beforeDevCommand = ~pkg_manager_run_command~ start~double-dash~ --port 1420
        beforeBuildCommand = ~pkg_manager_run_command~ build # this comment should be stripped
        devPath = http://localhost:1420

        [files]
        tauri.svg = src/assets/tauri.svg
        styles.css = src/styles.css
    "#;

        let content = r#"{
    "build": {
        "beforeDevCommand": "~fragment_before_dev_command~",
        "beforeBuildCommand": "~fragment_before_build_command~",
        "devPath": "~fragment_dev_path~",
        "distDir": "~fragment_dist_dir~"
    },
}"#;

        let manifest = Manifest::parse(manifest_file, false).unwrap();
        assert_eq!(
            Template::replace_vars(content, "cta_lib", "cta-app", PackageManager::Npm, manifest)
                .as_str(),
            r#"{
    "build": {
        "beforeDevCommand": "npm run start -- --port 1420",
        "beforeBuildCommand": "npm run build",
        "devPath": "http://localhost:1420",
        "distDir": ""
    },
}"#
            .to_string()
        );

        let manifest = Manifest::parse(manifest_file, false).unwrap();
        assert_eq!(
            Template::replace_vars(
                content,
                "cta_lib",
                "cta-app",
                PackageManager::Pnpm,
                manifest
            )
            .as_str(),
            r#"{
    "build": {
        "beforeDevCommand": "pnpm start --port 1420",
        "beforeBuildCommand": "pnpm build",
        "devPath": "http://localhost:1420",
        "distDir": ""
    },
}"#
            .to_string()
        );
    }
}
