use crate::{generated::css_classes::C, samples};
use html_to_seed_lib::html_to_seed;
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlDocument, HtmlTextAreaElement};

// ------ ------
//     Model
// ------ ------

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

pub fn view(model: &Model) -> impl View<Msg> {
    div![
        class![
            C.bg_purple_600,
            C.flex,
            C.flex_col,
            C.h_screen,
            C.text_gray_900,
        ],
        div![
            class![C.flex, C.justify_between,],
            div![
                class![C.flex,],
                h1![
                    class![
                        C.font_bold,
                        C.px_4,
                        C.py_2,
                        C.text_white,
                        C.text_2xl,
                        C.tracking_wider,
                    ],
                    "Html to Seed",
                ],
            ],
            github_button()
        ],
        div![
            class![C.flex, C.h_full, C.mb_3,],
            div![
                class![C.flex, C.flex_col, C.ml_3, C.w_1of2,],
                div![
                    class![
                        C.relative,
                        C.bg_gray_200,
                        C.py_2,
                        C.rounded_t_lg,
                        C.text_center,
                    ],
                    sample_menu(),
                    div![
                        class![C.w_full,],
                        p![
                            class![C.py_2, C.text_sm,],
                            "Type or paste HTML fragment",
                        ],
                    ],
                ],
                textarea![
                    class![
                        C.border,
                        C.font_mono,
                        C.h_full,
                        C.leading_tight,
                        C.text_sm,
                    ],
                    attrs! { At::SpellCheck => "false", },
                    input_ev(Ev::Input, Msg::HtmlChanged),
                    model.html,
                ],
            ],
            div![
                class![C.flex, C.flex_col, C.ml_3, C.mr_3, C.w_1of2,],
                div![
                    class![C.bg_gray_200, C.flex, C.py_2, C.rounded_t_lg,],
                    div![
                        class![C.absolute, C.flex, C.ml_3, C.my_1,],
                        use_typed_classes_button(model.use_typed_classes),
                        copy_button(model.is_copy_supported),
                    ],
                    p![
                        class![C.py_2, C.text_center, C.text_sm, C.w_full,],
                        "Rust code compatible with Seed",
                    ],
                ],
                textarea![
                    class![
                        C.border,
                        C.font_mono,
                        C.h_full,
                        C.leading_tight,
                        C.text_sm,
                    ],
                    attrs! {
                        At::Id => "rustcode",
                        At::SpellCheck => "false",
                    },
                    model.rust,
                ],
            ],
        ],
    ]
}

fn copy_button(is_copy_supported: bool) -> Node<Msg> {
    button![
        class![
            C.border, C.border_gray_800, C.flex, C.hover__bg_gray_900, C.hover__text_white, C.ml_3, C.p_1, C.rounded, C.text_sm,
            C.invisible => !is_copy_supported
        ],
        copy_icon(),
        simple_ev(Ev::Click, Msg::CopyToClipboard),
        p![class! [ C.ml_1, C.mr_1 C.self_center, ], "Copy",],
    ]
}

fn copy_icon() -> Node<Msg> {
    svg![
        class![C.fill_current, C.h_4, C.ml_1, C.self_center, C.w_4,],
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
        class![
            C.border,
            C.border_white,
            C.flex,
            C.hover__bg_gray_800,
            C.hover__border_gray_800,
            C.items_center,
            C.mr_3,
            C.my_3,
            C.px_3,
            C.rounded,
            C.text_white,
            C.text_xs,
        ],
        attrs! {
            At::Href => "http://github.com",
            At::Target => "_blank",
            At::Rel => "noreferrer",
        },
        img![
            class! [ C.w_3 C.h_3, C.mr_1, ],
            attrs! {
                    At::Src => "../static/images/GitHub-Mark-Light-120px-plus.png",
                    At::Alt => "Github logo",
            },
        ],
        "Github",
    ]
}

fn sample_menu() -> Node<Msg> {
    nav![
        class![C.group, C.ml_2, C.relative,],
        div![
            class![
                C.absolute,
                C.cursor_pointer,
                C.flex,
                C.items_center,
                C.left_0,
                C.pl_4,
                C.py_2,
                C.text_sm,
                C.tracking_wider,
            ],
            "Samples",
            svg![
                class![C.fill_current, C.h_4, C.w_4,],
                attrs! {
                    At::ViewBox => "0 0 20 20",
                },
                path![
                    attrs! { At::D => "M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z", },
                ],
            ],
        ],
        ul![
            class![
                C.absolute,
                C.bg_white,
                C.group_hover__visible,
                C.invisible,
                C.items_center,
                C.mt_8,
                C.overflow_hidden,
                C.rounded_lg,
                C.shadow_lg,
                C.w_48,
            ],
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
    li![button![
        class![
            C.px_4,
            C.py_2,
            C.block,
            C.hover__text_white,
            C.hover__bg_gray_800,
            C.text_sm,
            C.w_full,
        ],
        simple_ev(Ev::Click, Msg::HtmlChanged(html.to_string())),
        label,
    ],]
}

fn use_typed_classes_button(use_typed_classes: bool) -> Node<Msg> {
    button![
        class![
            C.border, C.border_gray_800, C.flex, C.hover__bg_gray_900, C.hover__text_white, C.p_1, C.rounded, C.text_sm,
            C.bg_purple_700 => use_typed_classes,
            C.text_white => use_typed_classes,
        ],
        simple_ev(Ev::Click, Msg::ToggleUseTypedClasses),
        if use_typed_classes {
            check_mark_icon()
        } else {
            cross_icon()
        },
        p![class![C.ml_1, C.mr_1, C.self_center,], "Typed Classes",],
    ]
}

fn check_mark_icon() -> Node<Msg> {
    svg![
        class![C.fill_current, C.h_4, C.ml_1, C.self_center, C.w_4,],
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
        class![C.fill_current, C.h_4, C.ml_1, C.self_center, C.w_4,],
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
