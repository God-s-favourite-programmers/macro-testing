use module_registration::use_actions;

mod commands;

pub fn using_modules(input: &str) {
    let response = match input {
        //use_actions!(* => commands::*::do_stuff(),)
        "ping" => commands::ping::do_stuff(),
        "pong" => commands::pong::do_stuff(),
        _ => "Oh no".to_string()
    };
    //use_actions!({*}[=>](commands)*[do_stuff();])
    use_actions!((src/commands))
}