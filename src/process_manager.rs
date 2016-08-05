use output::OutputTrait;
use mio::tcp::*;
use ::SERVER;
use controller::ControllerHandler;

pub struct ProcessManager<'a, O : 'a> {
    output : &'a O,
    wait_for_slaves : bool,
    is_running : bool,
    in_reload : bool,
    in_shutdown : bool,
    bridge : String,
    app_bootstrap : String,
    appenv : String,
    debug : bool,
    logging : bool,
    handled_requests : u32,
    max_reqests : u32,
    timeout : i32,
    port : usize,
    host : String,
    slave_count : usize
}

impl<'a, O> ProcessManager<'a, O> where O : OutputTrait {
    pub fn new(
        output : &'a O,
        port : usize,
        host : String,
        slave_count : usize
    ) -> ProcessManager<'a, O> {
        ProcessManager {
            output : output,
            wait_for_slaves : true,
            is_running : false,
            in_reload : false,
            in_shutdown : false,
            bridge : "".to_string(),
            app_bootstrap : "".to_string(),
            appenv : "".to_string(),
            debug : true,
            logging : true,
            handled_requests : 0,
            max_reqests : 2000,
            timeout : 30,
            port : port,
            host : host,
            slave_count : slave_count,
        }
    }

    pub fn run(&self) {
        // mio for onSlaveConnection
        // mio for onWeb


        let controller_address = "0.0.0.0:5500".parse().unwrap();
        let controller_listener = TcpListener::bind(&controller_address).unwrap();

        self.output.writeln(&format!(
            "Starting PHP-PM (Native) with {} workers, using {} ...",
            self.slave_count,
            "[LOOP CLASS]"
        ));

        let mut event_loop = ::mio::EventLoop::new().unwrap();
        event_loop.register(
            &controller_listener,
            SERVER,
            ::mio::EventSet::all(),
            ::mio::PollOpt::all()
        );

        event_loop.run(&mut ControllerHandler {
            server: controller_listener
        });
    }
}
