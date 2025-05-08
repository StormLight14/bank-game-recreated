use dioxus::prelude::*;
use uuid::Uuid;
use crate::{Player, Page, PAGE, PLAYERS, PREFERENCES};

#[component]
pub fn Home() -> Element {
  let mut player_name_input = use_signal(|| String::new());

  let add_player = move |_| {
    if !player_name_input.read().is_empty() && !PLAYERS.read().iter().any(|p| p.name == *player_name_input.read()) {
      PLAYERS.write().push(Player {
        id: Uuid::new_v4().to_string(),
        name: player_name_input.read().clone(),
        score: 0,
        is_banking: false,
      });
      player_name_input.write().clear();
    }
  };

  let remove_player = move |player_id: String| {
    PLAYERS.write().retain(|p| p.id != player_id);
  };

  let new_player_data: Vec<(String, String)> = PLAYERS.read().iter().map(|p| (p.id.clone(), p.name.clone())).collect();

  rsx! {
    h1 { "BANK" }
    input { r#type: "range", value: "{PREFERENCES.read().rounds}", min: "5", max: "30", oninput: move |event| PREFERENCES.write().rounds = event.value().parse().unwrap() }
    p { "{PREFERENCES.read().rounds} rounds" }

    form { class: "row", onsubmit: add_player,
      input { r#type: "text", placeholder: "Player Name...", value: "{player_name_input}", oninput: move |event| player_name_input.set(event.value().clone()) }
      button { r#type: "submit", "Add" }
    }

    div { class: "players",
    for (player_id, player_name) in new_player_data {
      div { key: "{player_id}", class: "player",
        p { "{player_name}" }
        button { r#type: "button", class: "remove-player", onclick: move |_| remove_player(player_id.clone()),
          "Remove"
        }
      }
    }
    }

    div { class: "checkbox-container",
      input { r#type: "checkbox", id: "virtual-dice-checkbox", name: "virtual-dice-checkbox", checked: PREFERENCES.read().use_virtual_dice, onchange: move |event| PREFERENCES.write().use_virtual_dice = event.value().parse().unwrap() }
      label { r#for: "virtual-dice-checkbox", "Use Virtual Dice" }
    }

    div { class: "checkbox-container",
      input { r#type: "checkbox", id: "show-rolls-checkbox", name: "show-rolls-checkbox", checked: PREFERENCES.read().show_roll_count, onchange: move |event| PREFERENCES.write().show_roll_count = event.value().parse().unwrap() }
      label { r#for: "show-rolls-checkbox", "Show Roll Count" }
    }

    button { r#type: "button", onclick: move |_| *PAGE.write() = Page::Game, "Play!" }
  }
}
