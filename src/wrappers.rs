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
struct PlayerWrapper {
	base: qt_base_class!(trait QObject),
	player: Player,

	get_hand_count: qt_method!(fn get_hand_count(&self) -> usize {
		self.player.get_hand_count()
	}),

	get_hand_at: qt_method!(fn get_hand_at(&self, idx: usize) -> HandWrapper {
		let hand = self.player.get_hand_at(idx);
		HandWrapper { hand }
	})
}

#[derive(QObject,Default)]
struct HandWrapper {
	base: qt_base_class!(trait QObject),
	hand: Hand,

	get_card_count: qt_method!(fn get_card_count(&self) -> usize {
		self.hand.get_card_count()
	}),

	get_card_at: qt_method!(fn get_card_at(&self, idx: usize) -> CardWrapper {
		let card = self.hand.get_card_at(idx);
		CardWrapper { card }
	})
}

#[derive(QObject,Default)]
struct CardWrapper {
	base: qt_base_class!(trait QObject),
	card: Card,

	to_u32: qt_method!(fn to_u32(&self) -> u32 {
		self.card.to_u32()
	})
}
