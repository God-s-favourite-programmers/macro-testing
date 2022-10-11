use module_registration::import;

mod commands;

pub fn using_modules() {
    commands::ping::do_stuff();
    commands::pong::do_stuff();
}