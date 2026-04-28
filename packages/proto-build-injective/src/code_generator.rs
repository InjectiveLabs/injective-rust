use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, io};

use log::info;
use prost::Message;
use prost_types::FileDescriptorSet;
use walkdir::WalkDir;

use crate::{mod_gen, transform};

const UNSUPPORTED_MODULE: &[&str] = &[
    // currently unsupported due to dependency on tendermint-proto
    "cosmos.base.abci",
    "cosmos.base.kv",
    "cosmos.base.reflection",
    "cosmos.base.store",
    "cosmos.base.snapshots",
    "cosmos.base.tendermint",
];

#[derive(Clone, Debug)]
pub struct CosmosProject {
    pub name: String,
    pub version: String,
    pub project_dir: String,

    /// determines which modules to include from the project
    pub include_mods: Vec<String>,
}

pub struct CodeGenerator {
    project: CosmosProject,
    root: PathBuf,
    out_dir: PathBuf,
    tmp_build_dir: PathBuf,
    deps: Vec<CosmosProject>,
}

impl CodeGenerator {
    pub fn new(
        out_dir: PathBuf,
        // TODO: remove tmp_build_dir from constructor in favor of generated tmp dir
        tmp_build_dir: PathBuf,
        project: CosmosProject,
        deps: Vec<CosmosProject>,
    ) -> Self {
        Self {
            project,
            root: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            out_dir,
            tmp_build_dir,
            deps,
        }
    }

    pub fn generate(&self) {
        println!("Generating code for {}", self.project.name);
        self.prepare_dir();
        println!("dir prepared");
        self.compile_proto();
        println!("proto compiled");

        info!("🧪 [{}] Embellishing modules to expose nice API for library user...", self.project.name);

        self.exclude_unsupported_module();
        println!("unsupported modules excluded");
        self.transform();
        println!("transformed");
        self.generate_mod_file();
        println!("mod file generated");
        self.fmt();
        println!("fmt");

        info!("✨  [{}] Library is successfully generated!", self.project.name);
    }

    fn prepare_dir(&self) {
        if self.tmp_build_dir.exists() {
            remove_dir_all(self.tmp_build_dir.clone()).unwrap();
        }
        create_dir_all(self.tmp_namespaced_dir()).unwrap();
        output_version_file(&self.project.name, &self.project.version, &self.tmp_namespaced_dir());
    }

    fn exclude_unsupported_module(&self) {
        for entry in WalkDir::new(self.tmp_namespaced_dir()) {
            // println!("entry {:?}", entry);
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let filename = entry.file_name().to_os_string().to_str().unwrap().to_string();
                if UNSUPPORTED_MODULE.iter().any(|module| filename.contains(module)) {
                    fs::remove_file(entry.path()).unwrap();
                }
            }
        }
    }

    fn generate_mod_file(&self) {
        mod_gen::generate_mod_file(&self.absolute_out_dir());
    }

    fn transform(&self) {
        transform::copy_and_transform_all(&self.tmp_namespaced_dir(), &self.absolute_out_dir(), &self.file_descriptor_set());
    }

    fn absolute_out_dir(&self) -> PathBuf {
        self.root.join(&self.out_dir)
    }

    fn fmt(&self) {
        let manifest_path = find_cargo_toml(&self.absolute_out_dir());
        println!("manifest_path {:?}", manifest_path);
        let exit_status = Command::new("cargo")
            .arg("fmt")
            .arg("--manifest-path")
            .arg(manifest_path.to_string_lossy().to_string())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        if !exit_status.success() {
            panic!("unable to format with: cargo fmt --manifest-path {}", manifest_path.to_string_lossy());
        }
    }

    fn compile_proto(&self) {
        let buf_gen_template = self.root.join("buf.gen.yaml");

        let all_related_projects = [self.deps.clone(), vec![self.project.clone()]].concat();

        info!("🧪 [{}] Compiling types from protobuf definitions...", self.project.name);

        // Compile proto files for each file in `protos` variable
        // `buf generate —template {<buf_gen_template} <proto_file>`
        for project in all_related_projects {
            println!("\n=========================\nProject {:?}\n", project);

            let buf_root = WalkDir::new(self.root.join(&project.project_dir))
                .into_iter()
                .filter_map(|e| e.ok())
                .find(|e| {
                    println!("File Names {:?}", e);
                    e.file_name()
                        .to_str()
                        .map(|s| {
                            (s == "buf.yaml" || s == "buf.yml")
                                && !e
                                    .path()
                                    .parent()
                                    .unwrap()
                                    .to_path_buf()
                                    .to_str()
                                    .unwrap_or_default()
                                    .contains("dependencies/cosmos-sdk/core/internal")
                        })
                        .unwrap_or(false)
                })
                .map(|e| {
                    println!("Bug Path: {:?}", e.path().parent().unwrap().to_path_buf());
                    e.path().parent().unwrap().to_path_buf()
                })
                .unwrap();

            println!("\nPath to buf.yaml: {:?}\n", buf_root);

            let proto_path = &self.root.join(&project.project_dir).join("proto");

            println!("\nPath to ./proto/: {:?}\n", proto_path);

            let include_mod_groups = split_include_mod_groups(&project.include_mods);

            for (group_idx, include_mods) in include_mod_groups.iter().enumerate() {
                let mut cmd = Command::new("buf");
                cmd.arg("generate")
                    .arg(buf_root.to_string_lossy().to_string())
                    .arg("--template")
                    .arg(buf_gen_template.to_string_lossy().to_string())
                    .arg("--output")
                    .arg(self.tmp_namespaced_dir().to_string_lossy().to_string());

                append_include_paths(&mut cmd, proto_path, &project.name, include_mods);

                println!("\ncmd {:?}\n", cmd);

                let exit_status = cmd.spawn().unwrap().wait().unwrap();

                if !exit_status.success() {
                    panic!("unable to generate with: {:?}", cmd.get_args().collect::<Vec<_>>());
                }

                let mut cmd = Command::new("tree");
                cmd.arg("-L").arg("3").arg(self.tmp_namespaced_dir());
                cmd.spawn().unwrap().wait().unwrap();

                let descriptor_file = descriptor_file_path(self.tmp_namespaced_dir(), &project.name, group_idx, include_mod_groups.len());

                // generate descriptor file with `buf build buf.yaml --as-file-descriptor-set -o {descriptor_file}`
                let mut cmd = Command::new("buf");
                cmd.arg("build")
                    .arg(buf_root.to_string_lossy().to_string())
                    .arg("--as-file-descriptor-set")
                    .arg("-o")
                    .arg(descriptor_file.to_string_lossy().to_string());

                append_include_paths(&mut cmd, proto_path, &project.name, include_mods);

                println!("\ncmd {:?}", cmd);

                let exit_status = cmd.spawn().unwrap().wait().unwrap();

                if !exit_status.success() {
                    panic!("unable to build with: {:?}", cmd.get_args().collect::<Vec<_>>());
                }

                let mut cmd = Command::new("tree");
                cmd.arg("-L").arg("3").arg(self.tmp_namespaced_dir());
                cmd.spawn().unwrap().wait().unwrap();
            }
        }

        info!("✨  [{}] Types from protobuf definitions is compiled successfully!", self.project.name);
    }

    pub fn file_descriptor_set(&self) -> FileDescriptorSet {
        // list all files in self.tmp_namespaced_dir()
        let files = fs::read_dir(self.tmp_namespaced_dir())
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        // filter only files that match "descriptor_*.bin"
        let descriptor_files = files
            .iter()
            .filter(|f| f.file_name().unwrap().to_str().unwrap().starts_with("descriptor_"))
            .collect::<Vec<_>>();

        // read all files and merge them into one FileDescriptorSet
        let mut file_descriptor_set = FileDescriptorSet { file: vec![] };
        for descriptor_file in descriptor_files {
            let descriptor_bytes = &fs::read(descriptor_file).unwrap()[..];
            let mut file_descriptor_set_tmp = FileDescriptorSet::decode(descriptor_bytes).unwrap();
            file_descriptor_set.file.append(&mut file_descriptor_set_tmp.file);
        }

        file_descriptor_set
    }

    fn tmp_namespaced_dir(&self) -> PathBuf {
        self.tmp_build_dir.join(&self.project.name)
    }
}

fn output_version_file(project_name: &str, versions: &str, out_dir: &Path) {
    let path = out_dir.join(format!("{}_COMMIT", project_name.to_uppercase()));
    fs::write(path, versions).unwrap();
}

fn split_include_mod_groups(include_mods: &[String]) -> Vec<Vec<String>> {
    if include_mods.is_empty() {
        return vec![vec![]];
    }

    let (file_mods, dir_mods): (Vec<String>, Vec<String>) = include_mods.iter().cloned().partition(|include_mod| include_mod.ends_with(".proto"));

    let mut groups = Vec::new();
    if !dir_mods.is_empty() {
        groups.push(dir_mods);
    }
    if !file_mods.is_empty() {
        groups.push(file_mods);
    }
    groups
}

fn append_include_paths(cmd: &mut Command, proto_path: &Path, project_name: &str, include_mods: &[String]) {
    for include_mod in include_mods {
        cmd.arg("--path").arg(proto_path.join(project_name).join(include_mod));
    }
}

fn descriptor_file_path(tmp_namespaced_dir: PathBuf, project_name: &str, group_idx: usize, group_count: usize) -> PathBuf {
    if group_count == 1 {
        tmp_namespaced_dir.join(format!("descriptor_{}.bin", project_name))
    } else {
        tmp_namespaced_dir.join(format!("descriptor_{}_{}.bin", project_name, group_idx))
    }
}

fn find_cargo_toml(path: &Path) -> PathBuf {
    if path.join("Cargo.toml").exists() {
        path.to_path_buf().join("Cargo.toml")
    } else {
        find_cargo_toml(path.parent().expect("Cargo.toml not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::split_include_mod_groups;

    #[test]
    fn split_include_mod_groups_separates_proto_files_from_directory_paths() {
        let include_mods = vec![
            "auction".to_string(),
            "common/vouchers/v1/vouchers.proto".to_string(),
            "exchange".to_string(),
        ];

        let groups = split_include_mod_groups(&include_mods);

        assert_eq!(
            groups,
            vec![
                vec!["auction".to_string(), "exchange".to_string()],
                vec!["common/vouchers/v1/vouchers.proto".to_string()],
            ]
        );
    }

    #[test]
    fn split_include_mod_groups_keeps_proto_files_in_one_group() {
        let include_mods = vec![
            "auth".to_string(),
            "staking/v1beta1/genesis.proto".to_string(),
            "staking/v1beta1/staking.proto".to_string(),
            "staking/v1beta1/tx.proto".to_string(),
        ];

        let groups = split_include_mod_groups(&include_mods);

        assert_eq!(
            groups,
            vec![
                vec!["auth".to_string()],
                vec![
                    "staking/v1beta1/genesis.proto".to_string(),
                    "staking/v1beta1/staking.proto".to_string(),
                    "staking/v1beta1/tx.proto".to_string(),
                ],
            ]
        );
    }
}
