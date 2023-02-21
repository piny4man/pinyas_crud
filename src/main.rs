use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
// use surf;

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
    // let mut selected_pinya: &UseState<Option<Pinya>> = use_state(&cx, || None);

    let pinyas = use_future(&cx, (), |_| async move {
        reqwest::get("https://pinyaend.shuttleapp.rs/pinyas")
            .await
            .unwrap()
            .json::<Vec<Pinya>>()
            .await
    });

    // let get_pinya_by_id = move |id: &str| {
    //     cx.spawn({
    //         async move {
    //             let response
    //         }
    //     })
    // };

    let create_pinya = move |_| {
        cx.spawn({
            let new_pinya = new_pinya.to_owned();

            async move {
                let body = Pinya {
                    id: format!("{:?}", Utc::now().timestamp()), //id: "asdfasdfasdfasdfasdfasdf".trim().to_lowercase().to_string(), //format!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()),
                    alias:  new_pinya.get().to_string(),
                };

                // let response = surf::post("https://pinyaend.shuttleapp.rs/pinyas")
                //     .body_json(&body)
                //     .await;

                let response = reqwest::Client::new()
                    .post("https://pinyaend.shuttleapp.rs/pinyas")
                    .json(&body)
                    .fetch_mode_no_cors()
                    .send()
                    .await;

                match response {
                    Ok(_data) => {
                        println!("Pinya created!!!");
                        new_pinya.set("".to_string());
                    }
                    Err(_err) => {
                        println!("Pinya creation failed, please try again")
                    }
                }
            }
        });
    };

    let pinyas_list = cx.render(
        match pinyas.value() {
            Some(Ok(val)) => rsx!(val.iter().map(|pinya| rsx!(
                li {
                    // onclick: move |_|  get_pinya_by_id(&pinya.id),
                    "{pinya.alias}"
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
                            onclick: create_pinya,
                            "Create"
                        }
                        button {
                            // hidden: true,
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

            article {
                header {
                    h1 {
                        "Selected pinya"
                    }
                    div {

                    }
                }
            }
        }
    })
}
