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

extern crate blackjack;
use blackjack::card::card::*;
use blackjack::player::player::*;

use qmetaobject::*;

mod qrc;

#[derive(QObject,Default)]
struct BlackjackBackend {
	base: qt_base_class!(trait QObject),
	deck: Deck,
	player_count: usize,
	players: Vec<Player>,
	dealer: Player,
	currentPlayer: u32,

	initialize: qt_method!(fn initialize(&mut self) {
		self.dealer = Player::new(String::from("Dealer"), true, -1);
	}),

	new_game: qt_method!(fn new_game(&mut self, deck_count: usize, names: Vec<&str>, balances: Vec<u32>) {
		self.deck = Deck::new(deck_count);
		self.players.clear();
		for (name, balance) in names.iter().zip(balances.iter()) {
			let player = Player::new(name, false, balance);
			self.players.push(player);
		}
	}),

	begin_turn: qt_method!(fn begin_turn(&self, wager: u32) {
		let mut player = self.players[self.currentPlayer];
		if self.currentPlayer == 0 {
			self.dealer.game_over(0);
			self.dealer.bet(0, &mut self.deck);
			self.deck.reset();
		}
		player.bet(wager, &mut self.deck);
	}),

	pass_turn: qt_method!(fn pass_turn(&mut self) {
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
				true => dealer.play_as_dealer(&mut self.deck),
				false => 0
			};
			for player in self.players.iter_mut() {
				player.game_over(dealer_value);
			}
			dealer.game_over(0);
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
		let player = self.players[self.currentPlayer];
		player.hit(&mut self.deck);
	}),

	player_stand: qt_method!(fn player_stand(&self) {
		let player = self.players[self.currentPlayer];
		player.stand();
	}),

	player_double: qt_method!(fn player_double(&self) {
		let player = self.players[self.currentPlayer];
		player.double(&mut self.deck);
	}),

	player_split: qt_method!(fn player_split(&self) {
		let player = self.players[self.currentPlayer];
		player.split(&mut self.deck);
	}),

	player_surrender: qt_method!(fn player_surrender(&self) {
		let player = self.players[self.currentPlayer];
		player.surrender();
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
	qml_register_type::<PlayerWrapper>(cstr!("PlayerWrapper"), 1, 0, cstr!("PlayerWrapper"));
	qml_register_type::<HandWrapper>(cstr!("HandWrapper"), 1, 0, cstr!("HandWrapper"));
	qml_register_type::<CardWrapper>(cstr!("CardWrapper"), 1, 0, cstr!("CardWrapper"));
	let mut engine = QmlEngine::new();
	engine.load_file("qrc:/qml/Main.qml".into());
	engine.exec();
}
