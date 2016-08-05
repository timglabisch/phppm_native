struct ProcessManager {
    wait_for_slaves : bool,
    is_running : bool,
    in_reload : bool,
    in_shutdown : bool,
    bridge : String,
    app_bootstrap : String,
    appenv : String,
    debug : bool,
    logging : bool,
    handled_requests : u32
    max_reqests : u32
    timeout : i32
}

impl ProcessManager {
    
}
