use libremarkable::appctx::ApplicationContext;
use libremarkable::input::gpio::GPIOEvent;
use libremarkable::input::wacom::WacomEvent;
use libremarkable::input::multitouch::MultitouchEvent;
use libremarkable::ui_extensions::element::{UIElementWrapper, UIConstraintRefresh, UIElementHandle, UIElement};
use libremarkable::cgmath::Point2;
use libremarkable::framebuffer::common::color;


fn on_button(context: &mut ApplicationContext, event: GPIOEvent) {
    println!("BUTTON!")
}


fn on_wacom(context: &mut ApplicationContext, event: WacomEvent) {
    println!("PEN!")
}


fn on_touch(context: &mut ApplicationContext, event: MultitouchEvent) {
    println!("TOUCH!")
}


fn main() {
    let mut context = ApplicationContext::new(on_button, on_wacom, on_touch);
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
    //context.display_text(Point2 { x: 100.0, y: 100.0 }, color::BLACK, 30.0, 0, 0, "Hello World!".to_owned(), UIConstraintRefresh::RefreshAndWait);
    context.dispatch_events(true, true, true);
}
