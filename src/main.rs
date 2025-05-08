#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dioxus::prelude::*;
use components::{home, game, end};

mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone)]
pub struct Player {
  pub id: String,
  pub name: String,
  pub score: i64,
  pub is_banking: bool
}

#[derive(Clone)]
pub struct Preferences {
  pub rounds: u8,
  pub use_virtual_dice: bool,
  pub show_roll_count: bool
}

pub enum Page {
  Home,
  Game,
  End
}

static PAGE: GlobalSignal<Page> = Global::new(|| Page::Home);
static PLAYERS: GlobalSignal<Vec<Player>> = Global::new(|| Vec::new());
static WINNERS: GlobalSignal<Vec<Player>> = Global::new(|| vec![Player { id: "no id".to_owned(), name: "NO ONE".to_owned(), score: 0, is_banking: false}] );
static PREFERENCES: GlobalSignal<Preferences> = Global::new(|| Preferences { rounds: 10, use_virtual_dice: false, show_roll_count: true} );

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }

    div { class: "container",
      match *PAGE.read() {
        Page::Home => rsx! { home::Home {} },
        Page::Game => rsx! { game::Game {} },
        Page::End => rsx! { end::End {} },
      }
    }
  }
}
