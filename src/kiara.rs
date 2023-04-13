use std::process::Stdio;
use viva::{EnvCheckStrategy, PkgInstallStrategy, VivaEnv, VivaGlobals};
use serde::{Deserialize, Serialize};
use crate::defaults::{ALL_KIARA_PACKAGES, KIARA_CONDA_CHANNELS};
use std::str::FromStr;
use tokio::process::Command;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KiaraContext {
    pub name: String,
    pub viva_env: VivaEnv
}

impl KiaraContext {
    pub fn create<S: AsRef<str>>(name: &str, plugins: Option<Vec<S>>, globals: &VivaGlobals) -> Self {

        // let specs = vec!("xxx");

        let specs = match plugins {
            Some(plugins) => {
                let specs = plugins.iter().map(|p| String::from_str(p.as_ref()).unwrap()).collect();
                specs
            },
            None => {
                let default_strings_vec: Vec<String> = ALL_KIARA_PACKAGES.iter().map(|s| s.to_string()).collect();
                default_strings_vec
            }
        };

        let conda_channels = KIARA_CONDA_CHANNELS.iter().map(|s| s.to_string()).collect();

        let viva_env = VivaEnv::create(name.clone(), Some(specs), Some(conda_channels), globals).expect(format!("Failed to create environment: {}", name).as_str());

        KiaraContext {
            name: String::from(name),
            viva_env: viva_env
        }
    }

    /// Create the command instance to run the kiara command (in the kiara context).
    ///
    /// In case the conda environment is not yet available, it will be created first.
    pub async fn create_kiara_command<S: AsRef<str>, I: AsRef<[S]>>(&self, sub_command: I) -> anyhow::Result<Command> {
        let env_check_strategy: EnvCheckStrategy = EnvCheckStrategy::Auto;
        let pkg_install_strategy: PkgInstallStrategy = PkgInstallStrategy::Append;

        let mut full_command = vec!("kiara".to_string());
        full_command.extend(sub_command.as_ref().iter().map(|s| s.as_ref().to_string()));

        let command = self.viva_env.create_command_in_env(&full_command, env_check_strategy, pkg_install_strategy);

        command.await

    }

    pub async fn run_kiara_command<S: AsRef<str>, I: AsRef<[S]>>(&self, sub_command: I) -> anyhow::Result<()> {

        let mut command = self.create_kiara_command(&sub_command).await?;

        let child = command
            .stdout(Stdio::piped())
            .spawn()
            .expect(format!("Failed to spawn kiara subcommand: {}", &sub_command.as_ref().iter().map(|s| s.as_ref()).collect::<Vec<&str>>().join(" ")).as_str());

        let output = child.wait_with_output().await?;
        // unsafe { child.detach() };

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Output: {}", stdout);
        } else {
            eprintln!("Error: {:?}", output);
        }

        Ok(())
    }

}
