use cnctd_shell::Shell;

pub struct NPM;

impl NPM {
    pub async fn bump_version(version_part: &str) -> anyhow::Result<()> {
        let command = format!("npm version {}", version_part);
        Shell::run(&command, true).await?;
        Ok(())
    }
}