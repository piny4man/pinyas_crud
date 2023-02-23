use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use reqwest;
// use surf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Pinya {
    pub id: String,
    pub alias: String
}

fn main() {
    dioxus_web::launch(app);
}

#[inline_props]
fn PinyaItem<'a>(cx: Scope<'a>, name: &'a str) -> Element {
    cx.render(rsx! {
        li { "{name}" }
    })
}

#[inline_props]
fn Button<'b>(cx: Scope<'b>, label: &'b str, class: &'b str, onclick: EventHandler<'b, MouseEvent>) -> Element {
    cx.render(rsx! {
        button {
            class: "{class}",
            onclick: move |event| onclick.call(event),
            // onclick: move |event| on_click.call(event),
            "{label}"
        }
    })
}

fn app(cx: Scope) -> Element {
    let new_pinya = use_state(&cx, || "".to_string());
    // let mut selected_pinya: &UseState<Option<Pinya>> = use_state(&cx, || None);

    let get_pinyas = use_future(&cx, (), |_| async move {
        reqwest::get("https://pinyaend.shuttleapp.rs/pinyas")
            .await
            .unwrap()
            .json::<Vec<Pinya>>()
            .await
    });

    let create_pinya = move |_| {
        cx.spawn({
            let new_pinya = new_pinya.to_owned();

            async move {
                let body = Pinya {
                    id: format!("{:?}", Utc::now().timestamp()),
                    alias:  new_pinya.get().to_string(),
                };

                let response = reqwest::Client::new()
                    .post("https://pinyaend.shuttleapp.rs/pinyas")
                    .json(&body)
                    .send()
                    .await;

                match response {
                    Ok(_data) => {
                        println!("Pinya created!!!");
                        new_pinya.set("".to_string())
                    }
                    Err(_err) => {
                        println!("Pinya creation failed, please try again")
                    }
                }
            }
        });
    };

    let pinyas_list = cx.render(
        match get_pinyas.value() {
            Some(Ok(val)) => rsx!(val.iter().map(|pinya| rsx!(
                PinyaItem {
                    key: "{pinya.id}",
                    // onclick: move |_|  get_pinya_by_id(&pinya.id),
                    name: "{pinya.alias}"
                }
            ))),
            Some(Err(_err)) => rsx!("Something went wrong, no pinyas here :("),
            None => rsx!("Fetching all pinyas"),
        }
    );

    cx.render(rsx! {
        main {
            style { include_str!("./styles.css") }
            article {
                class: "create--container",
                header {
                    h1 {
                        "Piny4man viewers"
                    }
                    section {
                        input {
                            placeholder: "User alias",
                            oninput: move |evt| new_pinya.set(evt.value.clone())
                        }
                        Button {
                            label: "Create comp",
                            onclick: create_pinya,
                            class: if new_pinya.get().is_empty() { "disabled" } else { "" }
                        }
                        Button {
                            label: "Refresh List",
                            onclick: move |_| get_pinyas.restart(),
                            class: ""
                        }
                    }

                    ul {
                        class: "pinyas--list",
                        pinyas_list
                    }
                }
            }

            // article {
            //     header {
            //         h1 {
            //             "Selected pinya"
            //         }
            //         div {

            //         }
            //     }
            // }
        }
    })
}
//https://www.youtube.com/watch?v=BH-SnQ8J1VU&list=PLfP6i5T0-DkIMLNRwmJpRBs4PJvxfgwBg