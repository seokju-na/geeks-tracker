#[cfg(test)]
pub mod git {
  use run_script::ScriptOptions;

  const BASE_PATH: &str = "test-fixtures";

  pub struct FixtureRepository {
    pub path: String,
    pub git_path: String,
  }

  impl FixtureRepository {
    pub fn open(name: &str, setup_script: &str) -> Self {
      let path = format!("{}/{}", BASE_PATH, name);

      let init_script = format!(
        r#"
            mkdir -p {}
            cd {}
            git init
            {}
            mv .git _git
            "#,
        path, path, setup_script
      );
      run_script::run(&init_script, &vec![], &ScriptOptions::new()).unwrap();

      Self {
        path: path.to_string(),
        git_path: format!("{}/_git", path.to_string()),
      }
    }
  }

  impl Drop for FixtureRepository {
    fn drop(&mut self) {
      let rm_script = format!("rm -rf {}", self.path);
      run_script::run(&rm_script, &vec![], &ScriptOptions::new()).unwrap();
    }
  }
}
