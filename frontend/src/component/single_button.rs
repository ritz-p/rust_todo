use std::rc::Rc;

use patternfly_yew::components::{button::Button, form::ActionGroup};
use web_sys::console;
use yew::{function_component, html, Callback, Html};

use crate::{props::single_button_props::SingleButtonProps, utils::{request::get::{GetArgs, GetArgsToJsValue}, wasm::invoke}};

#[function_component]
pub fn SingleButton(props: &SingleButtonProps) -> Html {
    let url = Rc::new(props.url.clone());
    let onclick = {
        Callback::from(move |_|{
                let args = GetArgs::url_to_js_value(url.to_string());
                match args{
                    Ok(serialized_args)=>{
                        console::log_1(&serialized_args);
                        // invoke("delete",serialized_args).await;
                    }
                    Err(_) => todo!(), 
                }
            }
        )
    };
    html!(
        <ActionGroup>
            <Button
                label={props.label.clone()}
                onclick={onclick}
                variant={props.button_variant}
                r#type={props.button_type}
            />
        </ActionGroup>
    )
}
