mod samples;
use html_to_seed_lib::html_to_seed;
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlDocument, HtmlTextAreaElement};

pub struct Model {
    html: String,
    rust: String,
    is_copy_supported: bool,
    use_typed_classes: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            html: String::default(),
            rust: String::default(),
            is_copy_supported: false,
            use_typed_classes: false,
        }
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let mut model = Model::default();
    model.is_copy_supported = html_document().query_command_supported("copy");
    model
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    HtmlChanged(String),
    CopyToClipboard,
    ToggleUseTypedClasses,
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::HtmlChanged(html) => {
            model.html = html;
            model.rust = html_to_seed(&model.html, model.use_typed_classes);
        },
        Msg::CopyToClipboard => {
            copy_to_clipboard();
        },
        Msg::ToggleUseTypedClasses => {
            model.use_typed_classes = !model.use_typed_classes;
            model.rust = html_to_seed(&model.html, model.use_typed_classes);
        },
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        C!["bg-purple-600 h-screen text-gray-900"],
        div![
            C!["grid grid-cols-2 "],
            div![
                C!["justify-self-start"],
                h1![
                    C!["font-bold px-4 py-2 text-white text-2xl tracking-wider"],
                    "Html to Seed",
                ],
            ],
            github_button()
        ],
        div![
            C!["grid grid-cols-2 h-full mb-3"],
            div![
                C!["ml-3 w-1of2"],
                div![
                    C!["relative bg-slate-100 py-2 rounded-t-lg text-center"],
                    sample_menu(),
                    div![
                        C!["full"],
                        p![C!["py-2 text-sm"], "Type or paste HTML fragment",],
                    ],
                ],
                textarea![
                    C!["border font-mono h-full w-full leading-tight text-sm"],
                    attrs! { At::SpellCheck => "false", },
                    input_ev(Ev::Input, Msg::HtmlChanged),
                    &model.html,
                ],
            ],
            div![
                C!["ml-3 mr-3 w-1of2"],
                div![
                    C!["bg-slate-100 flex py-2 rounded-t-lg"],
                    use_typed_classes_button(model.use_typed_classes),
                    copy_button(model.is_copy_supported),
                    p![
                        C!["py-2 text-center text-sm w-full"],
                        "Rust code compatible with Seed",
                    ],
                ],
                textarea![
                    C!["border font-mono h-full w-full leading-tight text-sm"],
                    attrs! {
                        At::Id => "rustcode",
                        At::SpellCheck => "false",
                    },
                    &model.rust,
                ],
            ],
        ],
    ]
}

fn copy_button(is_copy_supported: bool) -> Node<Msg> {
    button![
        C!["border border-gray-800 flex hover:bg-gray-900 hover:text-white ml-3 p-1 rounded text-sm",
            IF!(!is_copy_supported => "invisible")
        ],
        copy_icon(),
        ev(Ev::Click, |_| Msg::CopyToClipboard),
        p![C!["ml-1 mr-1 self-center"], "Copy",],
    ]
}

fn copy_icon() -> Node<Msg> {
    svg![
        C!["fill-current h-4 ml-1 self-center w-4"],
        attrs! {
            At::ViewBox => "0 0 20 20",
        },
        path![
            attrs! { At::D => "M6 6V2c0-1.1.9-2 2-2h10a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-4v4a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V8c0-1.1.9-2 2-2h4zm2 0h4a2 2 0 0 1 2 2v4h4V2H8v4zM2 8v10h10V8H2z", },
        ],
    ]
}

fn github_button() -> Node<Msg> {
    a![
        C!["justify-self-end border border-white flex hover:bg-gray-800 hover:border-gray-800 items-center mr-3 my-3 px-3 rounded text-white text-xs"],
        attrs! {
            At::Href => "http://github.com/phillipbaird/html-to-seed",
            At::Target => "_blank",
            At::Rel => "noreferrer",
        },
        img![
            C! ["w-3 h-3 mr-1"],
            attrs! {
                    At::Src => "./images/GitHub-Mark-Light-120px-plus.png",
                    At::Alt => "Github logo",
            },
        ],
        "Github",
    ]
}

fn sample_menu() -> Node<Msg> {
    nav![
        C!["group ml-2 relative"],
        div![
            C!["absolute cursor-pointer flex items-center left-0 pl-4 py-2 text-sm tracking-wider"],
            "Samples",
            svg![
                C!["fill-current h-4 w-4"],
                attrs! {
                    At::ViewBox => "0 0 20 20",
                },
                path![
                    attrs! { At::D => "M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z", },
                ],
            ],
        ],
        ul![
            C!["absolute bg-white group-hover:visible invisible items-center mt-8 overflow-hidden rounded-lg shadow-md w-48"],
            sample_menu_item(samples::HELLO_WORLD, samples::HELLO_WORLD_HTML),
            sample_menu_item(
                samples::BOOTSTRAP_NAVBAR,
                samples::BOOTSTRAP_NAVBAR_HTML
            ),
            sample_menu_item(samples::BULMA_BOX, samples::BULMA_BOX_HTML),
            sample_menu_item(samples::HTML_TO_SEED, samples::HTML_TO_SEED_HTML),
        ],
    ]
}

fn sample_menu_item(label: &str, html: &str) -> Node<Msg> {
    let html_string = html.to_string();
    li![button![
        C!["px-4 py-2 block hover:text-white hover:bg-gray-800 text-sm, w-full"],
        ev(Ev::Click, |_| Msg::HtmlChanged(html_string)),
        label,
    ],]
}

fn use_typed_classes_button(use_typed_classes: bool) -> Node<Msg> {
    button![
        C!["min-w-fit border border-gray-800 flex hover:bg-gray-900 hover:text-white ml-2 p-1 rounded text-sm",
           IF!(use_typed_classes => "bg-purple-700"),
           IF!(use_typed_classes => "text-white"),
        ],
        ev(Ev::Click, |_| Msg::ToggleUseTypedClasses),
        if use_typed_classes {
            check_mark_icon()
        } else {
            cross_icon()
        },
        p![C!["ml-1 mr-1 self-center"], "Typed Classes",],
    ]
}

fn check_mark_icon() -> Node<Msg> {
    svg![
        C!["fill-current h-4 ml-1 self-center w-4"],
        attrs! {
            At::ViewBox => "0 0 20 20",
        },
        path![
            attrs! { At::D => "M2.93 17.07A10 10 0 1 1 17.07 2.93 10 10 0 0 1 2.93 17.07zm12.73-1.41A8 8 0 1 0 4.34 4.34a8 8 0 0 0 11.32 11.32zM6.7 9.29L9 11.6l4.3-4.3 1.4 1.42L9 14.4l-3.7-3.7 1.4-1.42z", },
        ],
    ]
}

fn cross_icon() -> Node<Msg> {
    svg![
        C!["fill-current h-4 ml-1 self-center w-4"],
        attrs! {
            At::ViewBox => "0 0 20 20",
        },
        path![
            attrs! { At::D => "M2.93 17.07A10 10 0 1 1 17.07 2.93 10 10 0 0 1 2.93 17.07zm1.41-1.41A8 8 0 1 0 15.66 4.34 8 8 0 0 0 4.34 15.66zm9.9-8.49L11.41 10l2.83 2.83-1.41 1.41L10 11.41l-2.83 2.83-1.41-1.41L8.59 10 5.76 7.17l1.41-1.41L10 8.59l2.83-2.83 1.41 1.41z", },
        ],
    ]
}

// -------- --------
// Support Functions
// -------- --------
fn copy_to_clipboard() {
    rust_text_area().select();
    match html_document().exec_command("copy") {
        Ok(_) => (),
        Err(e) => log!(e),
    }
}

fn document() -> Document {
    seed::window().document().expect("Expected document.")
}

fn html_document() -> HtmlDocument {
    document()
        .dyn_into::<HtmlDocument>()
        .expect("Failed to convert Document to HtmlDocument.")
}

fn rust_text_area() -> HtmlTextAreaElement {
    let area = document()
        .get_element_by_id("rustcode")
        .expect("Could not find textarea with id #rustcode.");
    area.dyn_into::<HtmlTextAreaElement>()
        .expect("Failed to convert Element to HtmlTextAreaElement.")
}

// ------ ------
//     Start
// ------ ------

fn main() {
    App::start("app", init, update, view);
}
