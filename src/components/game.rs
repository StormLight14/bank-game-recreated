use dioxus::prelude::*;
use rand::Rng;
use crate::{Page, PAGE, PLAYERS, PREFERENCES, WINNERS};

static DICE_IMGS: [Asset; 6] = [asset!("assets/dice1.png"), asset!("assets/dice2.png"),asset!("assets/dice3.png"),asset!("assets/dice4.png"),asset!("assets/dice5.png"), asset!("assets/dice6.png")];

#[component]
pub fn Game() -> Element {
  let mut current_roll = use_signal(|| 1);
  let mut current_round = use_signal(|| 1);
  let mut current_score = use_signal(|| 0);

  let mut show_virtual_dice = use_signal(|| false);
  let mut show_roll_button = use_signal(|| true);
  let mut dice_one_value = use_signal(|| 1);
  let mut dice_two_value = use_signal(|| 1);
  let mut roll_label = use_signal(|| String::new());

  let mut round_end = move || {
    if *current_round.read() < PREFERENCES.read().rounds {
      *current_score.write() = 0;
      *current_roll.write() = 1;
      *current_round.write() += 1;

      for player in PLAYERS.write().iter_mut() {
        player.is_banking = false;
      }
    } else {
      *PAGE.write() = Page::End;

      for player in PLAYERS.write().iter_mut() {
        if player.score > WINNERS.read()[0].score {
          *WINNERS.write() = vec![player.clone()];
        } else if player.score == WINNERS.read()[0].score {
          WINNERS.write().push(player.clone());
        }
      }
    }
  };

  let mut bank = move |player_index: usize| {
    if *current_score.read() > 0 {
      PLAYERS.write()[player_index].is_banking = true;
      PLAYERS.write()[player_index].score += *current_score.read();

      let all_players_banked = !PLAYERS.read().iter().any(|p| !p.is_banking);
      if all_players_banked {
        round_end();
      }
    }
  };

  let undo_bank = move |player_index: usize| {
    PLAYERS.write()[player_index].is_banking = false;
    PLAYERS.write()[player_index].score -= *current_score.read();
  };

  let mut roll_pressed = move |button: u8| {
    if button == 7 && *current_roll.read() <= 3 {
      *current_score.write() += 70;
      *current_roll.write() += 1;
    } else if button == 0 && *current_roll.read() > 3 {
      *current_score.write() *= 2;
      *current_roll.write() += 1;
    } else if button == 7 {
      round_end();
    } else if button != 0 {
      *current_roll.write() += 1;
      *current_score.write() += button as i64;
    }
  };

  let mut roll_virtual_dice = move || {
    let mut rng = rand::rng();
    let rand_one: i64 = rng.random_range(1..=6);
    let rand_two: i64 = rng.random_range(1..=6);
    let roll = rand_one + rand_two;

    *dice_one_value.write() = rand_one;
    *dice_two_value.write() = rand_two;
    *show_virtual_dice.write() = true;

    if roll == 7 && *current_roll.read() <= 3 {
        *current_score.write() += 70;
        *current_roll.write() += 1;
        *roll_label.write() = "+70".to_owned();
    } else if dice_one_value == dice_two_value && *current_roll.read() > 3 {
        *current_score.write() *= 2;
        *current_roll.write() += 1;
        *roll_label.write() = "Doubles!".to_owned();
    } else if roll == 7 {
        round_end();
        *roll_label.write() = "7 ends the round!".to_owned();
    } else {
        *current_roll.write() += 1;
        *current_score.write() += roll;
        *roll_label.write() = format!("+{}", roll);
    }
};


  let new_player_data: Vec<(String, String, i64, bool)> = PLAYERS.read().iter().map(|p| (p.id.clone(), p.name.clone(), p.score.clone(), p.is_banking.clone())).collect();
  
  rsx! {
    p { class: "round", "Round: {current_round} / {PREFERENCES.read().rounds}"}
    p { class: "pot", "{current_score}"}
    for (i, (player_id, player_name, player_score, player_is_banking)) in new_player_data.iter().enumerate() {
      div { key: "{player_id}", class: "player",
        p { "{player_name}" }
        if *player_is_banking == false {
          button { class: "bank-button", onclick: move |_event| bank(i),
            "Bank!"
          }
        } else {
          button { class: "bank-button", onclick: move |_event| undo_bank(i),
            "Undo Bank!"
          }
        }
        p { "{player_score}" }
      }
    }

    if PREFERENCES.read().show_roll_count {
      p { "Rolls: {current_roll - 1}" }
    }

    if PREFERENCES.read().use_virtual_dice == false {
      div { class: "roll-menu",
        for num in 2..=12 {
          if num == 7 && *current_roll.read() > 3 {
            button { class: "roll-item bad-seven", onclick: move |_event| roll_pressed(num), "7" }
          } else if num == 12 && *current_roll.read() > 3 {
            button { class: "roll-item disabled", "12"}
          } else if num == 2 && *current_roll.read() > 3 {
            button { class: "roll-item disabled", "2" }
          } else {
            button { class: "roll-item", onclick: move |_event| roll_pressed(num), "{num}"}
          }
        }

        if *current_roll.read() > 3 {
          button { class: "roll-item", onclick: move |_event| roll_pressed(0), "Doubles"}
        } else {
          button { class: "roll-item disabled", "Doubles"}
        }
      }
    } else {
      if *show_virtual_dice.read() == true {
        div { class: "dice-container",
          img { src: DICE_IMGS[(*dice_one_value.read() as usize) - 1], alt: "Dice One with value of {dice_one_value} | " }
          img { src: DICE_IMGS[(*dice_two_value.read() as usize) - 1], alt: "Dice Two with value of {dice_two_value}" }
        }
        p { "{roll_label}" }
      }
      if *show_roll_button.read() == true {
        button { class: "roll-button", onclick: move |_event| roll_virtual_dice(), "Roll Dice" }
      }
    }
  }
}
