mod commands;
use module_registration::invoke;

pub fn using_modules(input: &str) {
    let response = match input {
        //use_actions!(* => commands::*::do_stuff(),)
        invoke!("src/commands" "do_stuff")
        _ => "Oh no".to_string()
    };
    //use_actions!({*}[=>](commands)*[do_stuff();])
}