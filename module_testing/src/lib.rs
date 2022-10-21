mod commands;
use module_registration::invoke;

pub fn using_modules(input: &str) {
    let response = invoke!("src/commands" "do_stuff");
    //use_actions!({*}[=>](commands)*[do_stuff();])
}
