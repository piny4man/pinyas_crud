use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime};

#[derive(Deserialize, Serialize, Debug)]
pub struct Pinya {
    pub id: String,
    pub alias: String
}

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let new_pinya = use_state(&cx, || "".to_string());
    

    let pinyas = use_future(&cx, (), |_| async move {
        reqwest::get("https://pinyaend.shuttleapp.rs/pinyas")
            .await
            .unwrap()
            .json::<Vec<Pinya>>()
            .await
    });
    let pinyas_list = cx.render(
        match pinyas.value() {
            Some(Ok(val)) => rsx!(val.iter().map(|pinya| rsx!(
                li { "{pinya.alias}" }
            ))),
            Some(Err(_err)) => rsx!("Something went wrong, no pinyas here :("),
            None => rsx!("Fetching all pinyas"),
        }
    );

    cx.render(rsx! {
        main {
            style { include_str!("./styles.css") }
            header {
                h1 {
                    "Piny4man viewers"
                }
                section {
                    class: "create--container",
                    input {
                        placeholder: "User alias",
                        oninput: move |evt| new_pinya.set(evt.value.clone())
                    }
                    button {
                        // onclick: move |_| cx.spawn({
                        //     let mut pinya = new_pinya.clone();
                        //     async move {
                        //         let body = Pinya {
                        //             id: format!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()),
                        //             alias: pinya.clone().to_string(),
                        //         };

                        //         let client = reqwest::Client::new();
                        //         client.post("https://pinyaend.shuttleapp.rs/pinyas")
                        //             .json(&body)
                        //             .send()
                        //             .await;
                        //         pinya.set("".to_string());
                        //     }
                        // }),
                        onclick: move |_| new_pinya.set("".to_string()),
                        "Create"
                    }
                    button {
                        onclick: move |_| pinyas.restart(),
                        "Refresh list"
                    }
                }

                ul {
                    class: "pinyas--list",
                    pinyas_list
                }
            }
        }
    })
}
