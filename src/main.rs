use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

struct Player {
  name: String,
  score: i64,
  is_banking: bool
}

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  let rounds = use_signal(|| 10);
  let player_name_input = use_signal(|| "");
  let use_virtual_dice = use_signal(|| false);
  let show_roll_count = use_signal(|| false);
  let mut players = use_signal(|| Vec::new());
  let mut winners = use_signal(|| Vec::new());
  let current_round = use_signal(|| 1);
  let current_roll = use_signal(|| 1);
  let current_score = use_signal(|| (0);
  let dice_one_value = use_signal(|| 1);
  let dice_two_value = use_signal(|| 1);
  let show_virtual_dice = use_signal(|| false);
  let show_roll_button = use_signal(|| true);
  let roll_label = use_signal(|| "");

  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
  }
}
