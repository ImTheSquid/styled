use leptos::leptos_dom::HydrationCtx;
use regex::Regex;
pub use stylist::{style, Result, Style};
pub extern crate leptos_meta;

#[macro_export]
macro_rules! view {
    ($cx: expr, $styles:expr, $($tokens:tt)*) => {{
        use $crate::leptos_meta::{Style, StyleProps};

        let v = $cx;
        let style = $styles;

        let $crate::StyleInfo { class_name, style_string } = $crate::get_style_info(style);

        view! {
            v,
            class={class_name.clone()},
            <Style>{style_string.clone()}</Style>
            $($tokens)*
        }
    }};
}

pub fn get_style_info(styles_result: Result<Style>) -> StyleInfo {
    let hydration_context_id = HydrationCtx::peek();

    let style_struct = styles_result.unwrap();

    let class_name = String::from("styled-") + &hydration_context_id.to_string();

    let style_string = style_struct.get_style_str().to_owned();

    style_struct.unregister();

    let re = Regex::new(r"stylist-\w+").unwrap();

    let style_string = re.replace_all(&style_string, &class_name);

    let re = Regex::new(r"(\.styled(-\d+)+) (\w+)").unwrap();

    let new_style_string = re.replace_all(&style_string, "$3$1").to_string();

    StyleInfo {
        class_name,
        style_string: new_style_string,
    }
}

#[derive(Clone)]
pub struct StyleInfo {
    pub class_name: String,
    pub style_string: String,
}
