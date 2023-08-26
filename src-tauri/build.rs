use dotenvy;

fn main() {

  // https://stackoverflow.com/a/73041609/6323666
  // using above to load in build script because dotenv macro cannot load custom path
  let dotenv_path = dotenvy::from_filename("../.env").expect("failed to find .env file");
  println!("cargo:rerun-if-changed={}", dotenv_path.display());

  for env_var in dotenvy::dotenv_iter().unwrap() {
      let (key, value) = env_var.unwrap();
      println!("cargo:rustc-env={key}={value}");
  }
  
  tauri_build::build()
}
