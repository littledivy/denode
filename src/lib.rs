use neon::prelude::*;
use neon::register_module;

use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_core::ModuleSpecifier;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

fn run_deno(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });

    let options = WorkerOptions {
        apply_source_maps: false,
        args: vec![],
        debug_flag: false,
        unstable: false,
        ca_filepath: None,
        user_agent: "hello_runtime".to_string(),
        seed: None,
        js_error_create_fn: None,
        create_web_worker_cb,
        attach_inspector: false,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        module_loader,
        runtime_version: "x".to_string(),
        ts_version: "x".to_string(),
        no_color: false,
        get_error_class_fn: Some(&get_error_class_name),
    };

    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("example.js");
    let main_module = ModuleSpecifier::resolve_path(&js_path.to_string_lossy()).unwrap();
    let permissions = Permissions::allow_all();

    let mut worker = MainWorker::from_options(main_module.clone(), permissions, &options);
    worker.bootstrap(&options);
    let _fut = async move {
        worker.execute_module(&main_module).await.unwrap();
        worker.run_event_loop().await.unwrap();
    };
    Ok(cx.undefined())
}

register_module!(mut m, { m.export_function("runDeno", run_deno) });
