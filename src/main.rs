use std::rc::Rc;
use std::env;
use deno_core::error::AnyError;
use deno_core::url::Url;

fn get_main_module(file_path: &str) -> Result<Url, AnyError> {
  let exe_path = env::current_exe().unwrap();
  let src_path = exe_path.parent().unwrap().parent().unwrap().parent().unwrap().join("src");
  let main_module = deno_core::resolve_path(file_path, &src_path)?;
  Ok(main_module)
}

async fn mustang_js(file_path: &str) -> Result<(), AnyError> {
  let main_module = get_main_module(file_path)?;
  let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
      module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
      ..Default::default()
  });

  let mod_id = js_runtime.load_main_module(&main_module, None).await?;
  let result = js_runtime.mod_evaluate(mod_id);
  js_runtime.run_event_loop(false).await?;
  result.await?
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(mustang_js("./input.js")) {
        eprintln!("error: {}", error);
    }
}