use libremarkable::appctx::ApplicationContext;
use libremarkable::input::gpio::GPIOEvent;
use libremarkable::input::wacom::WacomEvent;
use libremarkable::input::multitouch::MultitouchEvent;
use libremarkable::ui_extensions::element::{UIElementWrapper, UIConstraintRefresh, UIElementHandle, UIElement};
use libremarkable::cgmath::Point2;
use libremarkable::framebuffer::common::color;
use mango_client::mango::{MangoClient, Library};
use mango_client::opds::{OpdsClient, OpdsEntry};
use std::error::Error;
use std::env;
use std::io::{stdout, stdin};
use std::sync::Arc;
use std::io::Write;
use tokio::runtime::Runtime;
use tokio;

fn on_button(context: &mut ApplicationContext, event: GPIOEvent) {
    println!("BUTTON!");
}


fn on_wacom(context: &mut ApplicationContext, event: WacomEvent) {
    println!("PEN!");
}


fn on_touch(context: &mut ApplicationContext, event: MultitouchEvent) {
    println!("TOUCH!");
    context.remove_elements();
    context.clear(false);
    context.draw_elements();
}


fn get_or_query(env_name: &str, prompt_name: &str) -> String {
    if let Ok(env_value) = env::var(&env_name) {
        return env_value;
    }

    print!("{}: ", prompt_name);
    stdout().flush().unwrap();

    let mut input_line = String::new();
    stdin().read_line(&mut input_line).expect("Failed to read line");
    input_line = input_line.trim().to_owned();
    if input_line.len() == 0 {
        panic!("Line was empty!");
    }

    input_line
}

fn client() -> MangoClient {
    let base_url = get_or_query("MANGO_BASE_URL", "Enter Mango base url");
    let username = get_or_query("MANGO_USERNAME", "Enter Mango username");
    let password = get_or_query("MANGO_PASSWORD", "Enter Mango password");

    MangoClient::new(OpdsClient::new(&base_url, &username, &password))
}

fn add_titles(rt: &mut Runtime, context: &mut ApplicationContext, client: &MangoClient) -> Result<(), Box<dyn Error>>{
    let lib = rt.block_on(client.library())?;
    println!("{:#?}", lib);

    let mut i = 0;
    let mut y = 35;
    for entry in lib.entries.iter() {
        i += 1;
        println!("{} at y = {}", entry.title, y);
        context.add_element(format!("title_{}", i).as_str(), UIElementWrapper {
            refresh: UIConstraintRefresh::Refresh,
            position: Point2 { x: 50, y },
            onclick: None,
            inner: UIElement::Text {
                border_px: 0,
                text: String::from(entry.title.to_owned()),
                foreground: color::BLACK,
                scale: 30.0,
            },
            ..Default::default()
        });
        y += 30;
    }

    Ok(())
}

fn main() {
    let mut rt = Runtime::new().expect("Failed to create Tokio runtime");

    let client = client();

    /*rt.spawn(async {
        println!("spawned");
        let client = client();
        let lib = client.library().await.unwrap();
        println!("{:#?}", lib);
    });*/

    let mut context = ApplicationContext::new(on_button, on_wacom, on_touch);
    add_titles(&mut rt, &mut context, &client).expect("Failed to add titles.");

    context.clear(true);
    context.add_element("test", UIElementWrapper {
        refresh: UIConstraintRefresh::Refresh,
        position: Point2 { x: 100, y: 900 },
        onclick: None,
        inner: UIElement::Text {
            border_px: 0,
            text: String::from("Hello World!"),
            foreground: color::BLACK,
            scale: 250.0,
        },
        ..Default::default()
    });
    context.draw_elements();

    //title_page(&mut context, &client);

    //context.display_text(Point2 { x: 100.0, y: 100.0 }, color::BLACK, 30.0, 0, 0, "Hello World!".to_owned(), UIConstraintRefresh::RefreshAndWait);
    context.dispatch_events(true, true, true);
}
