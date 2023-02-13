use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
  cx.render(rsx!{
    main {
      style { include_str!("./styles.css") }
      header {
        h1 {
          "Piny4man viewers"
        }
        section {
          class: "create--container",
          input {
            placeholder: "User alias"
          }
          button {
            "Create"
          }
        }
      }
    }
  })
}
