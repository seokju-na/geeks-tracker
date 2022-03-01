use nanoid::nanoid;
use run_script::ScriptOptions;
use std::path::PathBuf;

pub struct FixtureRepository {
  pub path: String,
}

impl FixtureRepository {
  pub fn setup() -> Self {
    Self::setup_with_script("")
  }

  pub fn setup_with_script(setup_script: &str) -> Self {
    let path: PathBuf = ["test-fixtures", &nanoid!()].iter().collect();
    let path = path.to_str().unwrap();

    let init_script = format!(
      r#"
            mkdir -p {}
            cd {}
            git init
            git add .
            git commit --allow-empty -m "initial"
            {}
            "#,
      path, path, setup_script
    );
    run_script::run(&init_script, &vec![], &ScriptOptions::new()).unwrap();

    Self {
      path: path.to_owned(),
    }
  }
}

impl Drop for FixtureRepository {
  fn drop(&mut self) {
    let rm_script = format!("rm -rf {}", self.path);
    run_script::run(&rm_script, &vec![], &ScriptOptions::new()).unwrap();
  }
}
