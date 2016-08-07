use output::OutputTrait;
use mio::tcp::*;
use ::SERVER;
use controller::ControllerHandler;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::env;
use config::Config;

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
    config : &'a Config
}

impl<'a, O> ProcessManager<'a, O> where O : OutputTrait {
    pub fn new(
        output : &'a O,
        config : &'a Config
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
            config : config
        }
    }

    pub fn run(&self) {
        // mio for onSlaveConnection
        // mio for onWeb


        let controller_address = "0.0.0.0:5500".parse().unwrap();
        let controller_listener = TcpListener::bind(&controller_address).unwrap();

        self.output.writeln(&format!(
            "Starting PHP-PM (Native) with {} workers, using {} ...",
            self.config.workers,
            "[LOOP CLASS]"
        ));

        let mut event_loop = ::mio::EventLoop::new().unwrap();
        event_loop.register(
            &controller_listener,
            SERVER,
            ::mio::EventSet::all(),
            ::mio::PollOpt::all()
        );

        let slave_file_content = format!(
            include_str!("slave.php"),
            slave_port=5501,
            bootstrap="",
            bridge="",
            dir=self.config.working_directory
        );

        let mut file = File::create("/tmp/phppm_slave1").expect("failed to create /tmp/phppm_slave1");
        file.write_all(slave_file_content.as_bytes());

        Command::new("/Users/tim/.phpbrew/php/php-7.0.9/bin/php")
        .arg("/tmp/phppm_slave1")
        .stderr(Stdio::null())
        .spawn()
        .expect("php command failed to start");

        println!("loop!");

        event_loop.run(&mut ControllerHandler::new(controller_listener));
    }
}
