use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlElement, Node, Window};

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn window() -> Window {
    web_sys::window().unwrap()
}

fn document() -> Document {
    window().document().unwrap()
}

fn body() -> HtmlElement {
    document().body().unwrap()
}

fn create_element(identifier: &str, parent: &Node) -> Result<Element, JsValue> {
    let element = document().create_element(identifier)?;
    parent.append_child(&element)?;
    Ok(element)
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let canvas = create_element("canvas", &body())?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    canvas.style().set_property("position", "absolute")?;
    canvas.style().set_property("position", "absolute")?;
    canvas.style().set_property("left", "0px")?;
    canvas.style().set_property("top", "0px")?;
    body().style().set_property("overflow", "hidden")?;

    canvas.set_width(window().inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window().inner_height().unwrap().as_f64().unwrap() as u32);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        canvas.set_width(window().inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window().inner_height().unwrap().as_f64().unwrap() as u32);
        render(&context).unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn render(ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
    ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)?;

    // Draw the mouth.
    ctx.move_to(110.0, 75.0);
    ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)?;

    // Draw the left eye.
    ctx.move_to(65.0, 65.0);
    ctx.arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)?;

    // Draw the right eye.
    ctx.move_to(95.0, 65.0);
    ctx.arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)?;

    ctx.stroke();

    Ok(())
}
