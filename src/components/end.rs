use dioxus::prelude::*;
use std::cmp::Ordering;
use crate::{Page, Player, PAGE, PLAYERS, WINNERS};

#[component]
pub fn End() -> Element {
  let reset_game = move || {
    *PAGE.write() = Page::Home;
    *WINNERS.write() = vec![Player { id: "no id".to_owned(), name: "NO ONE".to_owned(), score: 0, is_banking: false}];
  };

  let mut new_player_data: Vec<(String, String, i64)> = PLAYERS.read().iter().map(|p| (p.id.clone(), p.name.clone(), p.score.clone())).collect();
  new_player_data.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
  
  rsx! {
    if WINNERS.len() == 1 {
      p { "{WINNERS.read()[0].name} has won the game with {WINNERS.read()[0].score} points!" }
    } else {
      p {
        for (i, winner) in WINNERS.read().iter().enumerate() {
          if i == WINNERS.read().len() - 2 {
            "{winner.name} and "
          }
          else if i != WINNERS.read().len() - 1 {
            "{winner.name}, "
          } else {
            "{winner.name} tied the game with {winner.score} points!"
          }
        }
      }
    }
    button { onclick: move |_| reset_game(), "Play Again!" }

    for (player_id, player_name, player_score) in new_player_data.iter() {
      div { key: "{player_id}", class: "player",
        p { "{player_name}" }
        p { "{player_score}" }
      }
    }
  }
}
