// Copyright (C) 2019 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3).

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

#[macro_use]
extern crate cstr;
#[macro_use]
extern crate cpp;
#[macro_use]
extern crate qmetaobject;
use qmetaobject::*;

use std::ffi::CStr;

extern crate blackjack;
use blackjack::card::card::*;
use blackjack::player::player::*;

mod qrc;

#[derive(QObject,Default)]
struct PlayerWrapper {
	base: qt_base_class!(trait QObject),
	/*player: Option<Player>,

	get_hand_count: qt_method!(fn get_hand_count(&self) -> usize {
		match self.player {
			None => 0,
			Some(player) => player.get_hand_count()
		}
	}),

	get_hand_at: qt_method!(fn get_hand_at(&self, idx: usize) -> Option<HandWrapper> {
		let player = self.player.unwrap()?;
		let hand = player.get_hand_at(idx);
		Some(HandWrapper { hand })
	})*/
}

#[derive(QObject,Default)]
struct HandWrapper {
	base: qt_base_class!(trait QObject),
	/*hand: Option<Hand>,

	get_card_count: qt_method!(fn get_card_count(&self) -> usize {
		match self.hand {
			None => 0,
			Some(hand) => hand.get_card_count()
		}
	}),

	get_card_at: qt_method!(fn get_card_at(&self, idx: usize) -> CardWrapper {
		match self.hand {
			None => CardWrapper { card: None },
			Some(hand) => {
				let card = hand.get_card_at(idx);
				CardWrapper { card: Some(card) }
			}
		}
	})*/
}

#[derive(QObject,Default)]
struct CardWrapper {
	base: qt_base_class!(trait QObject),
	/*card: Option<Card>,

	to_u32: qt_method!(fn to_u32(&self) -> u32 {
		match self.card {
			None => 0,
			Some(card) => card.to_u32()
		}
	}),

	to_image_name: qt_method!(fn to_image_name(&self) -> &CStr {
		let card = self.to_u32();
		if card == 0 {
			return cstr!("");
		}
		let suit = match card & 0b11110000 {
			DIAMONDS => "d",
			HEARTS => "h",
			CLUBS => "c",
			SPADES => "s"
		};
		let code = format!("{}{:02}.png", suit, card & 0b00001111);
		cstr!(code)
	})*/
}

#[derive(QObject,Default)]
struct BlackjackBackend {
	base: qt_base_class!(trait QObject),
	deck: Option<Deck>,
	player_count: usize,
	players: Vec<Player>,
	dealer: Option<Player>,
	currentPlayer: u32,

	initialize: qt_method!(fn initialize(&mut self) {
		self.dealer = Some(Player::new(String::from("Dealer"), true, -1));
	}),

	new_game: qt_method!(fn new_game(&mut self, deck_count: usize, names: Vec<&str>, balances: Vec<u32>) {
		self.deck = Some(Deck::new(deck_count));
		self.players.clear();
		for (name, balance) in names.iter().zip(balances.iter()) {
			let player = Player::new(name, false, balance);
			self.players.push(player);
		}
	}),

	begin_turn: qt_method!(fn begin_turn(&self, wager: u32) {
		if self.dealer == Some(dealer) && self.deck == Some(deck) {
			let mut player = self.players[self.currentPlayer];
			if self.currentPlayer == 0 {
				dealer.game_over(0);
				dealer.bet(0, &mut deck);
				deck.reset();
			}
			player.bet(wager, &mut deck);
		}
	}),

	pass_turn: qt_method!(fn pass_turn(&mut self) {
		if self.dealer == Some(dealer) && self.deck == Some(deck) {
			self.currentPlayer = (self.currentPlayer + 1) % self.playerCount;
			if self.currentPlayer == 0 {
				let mut dealer_plays = false;
				for player in self.players.iter() {
					if !player.has_lost() {
						dealer_plays = true;
						break;
					}
				}
				let dealer_value = match dealer_plays {
					true => dealer.play_as_dealer(&mut deck),
					false => 0
				};
				for player in self.players.iter_mut() {
					player.game_over(dealer_value);
				}
				dealer.game_over(0);
			}
		}
	}),

	player_can_surrender: qt_method!(fn player_can_surrender(&self) -> bool {
		self.players[self.currentPlayer].can_surrender_hand()
	}),

	player_can_split: qt_method!(fn player_can_split(&self) -> bool {
		self.players[self.currentPlayer].can_split_hand()
	}),

	player_is_playing: qt_method!(fn player_is_playing(&self) -> bool {
		self.players[self.currentPlayer].is_playing()
	}),

	player_hit: qt_method!(fn player_hit(&self) {
		if self.deck == Some(deck) {
			let player = self.players[self.currentPlayer];
			player.hit(&mut deck);
		}
	}),

	player_stand: qt_method!(fn player_stand(&self) {
		self.players[self.currentPlayer].stand();
	}),

	player_double: qt_method!(fn player_double(&self) {
		if self.deck == Some(deck) {
			self.players[self.currentPlayer].double(&mut deck);
		}
	}),

	player_split: qt_method!(fn player_split(&self) {
		if self.deck == Some(deck) {
			self.players[self.currentPlayer].split(&mut deck);
		}
	}),

	player_surrender: qt_method!(fn player_surrender(&self) {
		self.players[self.currentPlayer].surrender();
	})
}

fn main() {
	unsafe {
		cpp! { {
			#include <QtCore/QCoreApplication>
		}}
		cpp!{[]{
			 QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
		}}
	}
	QQuickStyle::set_style("Suru");
	qrc::load();
	qml_register_type::<BlackjackBackend>(cstr!("BlackjackBackend"), 1, 0, cstr!("BlackjackBackend"));
	//qml_register_type::<PlayerWrapper>(cstr!("PlayerWrapper"), 1, 0, cstr!("PlayerWrapper"));
	//qml_register_type::<HandWrapper>(cstr!("HandWrapper"), 1, 0, cstr!("HandWrapper"));
	//qml_register_type::<CardWrapper>(cstr!("CardWrapper"), 1, 0, cstr!("CardWrapper"));
	let mut engine = QmlEngine::new();
	engine.load_file("qrc:/qml/Main.qml".into());
	engine.exec();
}
