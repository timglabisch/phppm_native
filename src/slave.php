<?php
set_time_limit(0);

$bootstrap = '{bootstrap}';
$bridge = '{bridge}';

$dir = '{dir}';

$config = [
  'port' => {slave_port},
  'host' => '127.0.0.1',
  'session_path' => session_save_path(),
  'controllerHost' => 'tcp://127.0.0.1:5500',
  'app-env' => 'app_dev',
  'debug' => true,
  'logging' => true,
  'static' => true,
];

require_once file_exists($dir . '/vendor/autoload.php')
    ? $dir . '/vendor/autoload.php'
    : $dir . '/../../autoload.php';
//require_once $dir . '/functions.php';
//global for all global functions
\PHPPM\ProcessSlave::$slave = new \PHPPM\ProcessSlave($bridge, $bootstrap, $config);
\PHPPM\ProcessSlave::$slave->run();
