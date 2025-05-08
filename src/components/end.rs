use dioxus::prelude::*;
use crate::{Page, Player, PAGE, PLAYERS, WINNERS};

#[component]
pub fn End() -> Element {
  let reset_game = move || {
    *PAGE.write() = Page::Home;
    *WINNERS.write() = vec![Player { id: "no id".to_owned(), name: "NO ONE".to_owned(), score: 0, is_banking: false}];
  };

  rsx! {
    if WINNERS.len() == 1 {
      p { "{WINNERS.read()[0].name} has won the game with {WINNERS.read()[0].score} points!" }
    } else {
      p { "Players " }
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
    button { onclick: move |_| reset_game(), "Play Again!" }
  }
}
