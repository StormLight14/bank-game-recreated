use dioxus::prelude::*;
use uuid::Uuid;
use crate::{Player, Page, PAGE};

#[component]
pub fn Home() -> Element {
  let mut rounds = use_signal(|| 10);
  let mut player_name_input = use_signal(|| String::new());
  let mut use_virtual_dice = use_signal(|| false);
  let mut show_roll_count = use_signal(|| true);
  let mut players: Signal<Vec<Player>> = use_signal(|| Vec::new());

  let add_player = move |_| {
    if !player_name_input.read().is_empty() && !players.read().iter().any(|p| p.name == *player_name_input.read()) {
      players.write().push(Player {
        id: Uuid::new_v4().to_string(),
        name: player_name_input.read().clone(),
        score: 0,
        is_banking: false,
      });
      player_name_input.write().clear();
    }
  };

  let mut remove_player = move |player_id: String| {
    players.write().retain(|p| p.id != player_id);
  };

  let new_player_data: Vec<(String, String)> = players.read().iter().map(|p| (p.id.clone(), p.name.clone())).collect();

  rsx! {
    h1 { "BANK" }
    input { r#type: "range", value: "{rounds}", min: "5", max: "30", oninput: move |event| rounds.set(event.value().parse().unwrap()) }
    p { "{rounds} rounds" }

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
      input { r#type: "checkbox", id: "virtual-dice-checkbox", name: "virtual-dice-checkbox", checked: use_virtual_dice, onchange: move |event| use_virtual_dice.set(event.value().parse().unwrap()) }
      label { r#for: "virtual-dice-checkbox", "Use Virtual Dice" }
    }

    div { class: "checkbox-container",
      input { r#type: "checkbox", id: "show-rolls-checkbox", name: "show-rolls-checkbox", checked: show_roll_count, onchange: move |event| show_roll_count.set(event.value().parse().unwrap()) }
      label { r#for: "show-rolls-checkbox", "Show Roll Count" }
    }

    button { r#type: "button", onclick: move |_| *PAGE.write() = Page::Game, "Play!" }
  }
}
