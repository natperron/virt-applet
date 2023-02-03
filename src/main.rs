use std::env;

use gtk::prelude::*;
use getopts::Options;
use libappindicator::{AppIndicator, AppIndicatorStatus};
use virt::{connect::Connect, domain::Domain};

const APP_NAME: &str = "Virt Applet";

fn show_about_window() {
    let window_about = gtk::AboutDialog::new();
    window_about.set_program_name(APP_NAME);
    window_about.set_authors(env!("CARGO_PKG_AUTHORS").split(',').collect::<Vec<&str>>().as_ref());
    window_about.set_version(option_env!("CARGO_PKG_VERSION"));
    window_about.run();
    unsafe { window_about.destroy(); }
}

fn handle_error(e: virt::error::Error) {
    panic!("failed with code {}, message: {}", e.code, e.message)
}

fn get_virt_domains() -> Vec<Domain> {
    let mut domains: Vec<Domain> = Vec::new();
    match Connect::open("qemu:///system") {
        Ok(conn) => {
            match conn.list_all_domains(0) {
                Ok(arr) => { domains = arr; }
                Err(e) => handle_error(e)
            }
        }
        Err(e) => handle_error(e)
    }

    domains
}

fn start_virt_domain(domain: &Domain) {
    // domain.??
}

fn stop_virt_domain(domain: &Domain) {
    // domain.??
}

fn build_menu(indicator: &mut AppIndicator) -> gtk::Menu {
    let mut menu = gtk::Menu::new();
    for domain in get_virt_domains() {
        match domain.get_name() {
            Ok(domain_name) => {
                let menu_item_domain = gtk::MenuItem::with_label(domain_name.as_str());
                menu_item_domain.connect_activate(move |_| { start_virt_domain(&domain) });
                menu.append(&menu_item_domain);
            }
            Err(e) => handle_error(e)

        }
    }
    let separator1 = gtk::SeparatorMenuItem::new();
    let menu_item_about = gtk::MenuItem::with_label("About");
    menu_item_about.connect_activate(|_| { show_about_window(); });
    let separator2 = gtk::SeparatorMenuItem::new();
    let menu_item_quit = gtk::MenuItem::with_label("Quit");
    menu_item_quit.connect_activate(|_| { gtk::main_quit(); });


    menu.append(&separator1); 
    menu.append(&menu_item_about);
    menu.append(&separator2); 
    menu.append(&menu_item_quit);
    menu.show_all();

    indicator.set_menu(&mut menu);

    menu
}

fn build_indicator() -> AppIndicator {
    let mut indicator = AppIndicator::new(APP_NAME, "laptopconnected");
    indicator.set_status(AppIndicatorStatus::Active);
    indicator.set_icon("laptopdisconnected");

    indicator
}

fn run_main() {
    gtk::init().unwrap();
    let mut indicator = build_indicator();
    build_menu(&mut indicator);
    gtk::main();
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn run_opts() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    // opts.optopt("c", "config", "Use specified config file instead", "FILE");
    opts.optflag("h", "help", "Print this help menu and exit");
    opts.optflag("v", "version", "Print version info and exit");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { 
            println!("{}", f.to_string());
            print_usage(&program, opts);
            std::process::exit(1);    
        }
    };
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }
    if matches.opt_present("v") {
        println!("{} {}", APP_NAME, env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
}

fn main() {
    run_opts();

    run_main();
}
