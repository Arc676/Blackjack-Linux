// Copyright (C) 2019 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3)

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

#include "blackjackbackend.h"

BlackjackBackend::BlackjackBackend() {
	char* dealerName = (char*)malloc(7);
	sprintf(dealerName, "Dealer");
	dealer = player_new(dealerName, 1, -1);
}

void BlackjackBackend::newGame(int deckCount, int playerCount) {
	if (deck) {
		rust_freedeck(deck);
		for (Player** pplayer = players; pplayer < players + playerCount; pplayer++) {
			rust_freeplayer(*pplayer);
		}
		delete[] players;
	}
	deck = deck_new(deckCount);

	this->playerCount = playerCount;
	currentPlayer = 0;
	players = new Player*[playerCount];
	for (int i = 0; i < playerCount; i++) {
		char* name = (char*)malloc(12);
		sprintf(name, "Player %d", i + 1);
		players[i] = player_new(name, 0, 1000);
	}
}

void BlackjackBackend::beginTurn(int wager) {
}

void BlackjackBackend::passTurn() {
	currentPlayer = (currentPlayer + 1) % playerCount;
	if (currentPlayer == 0) {
		bool dealerPlays = false;
		for (Player** pplayer = players; pplayer < players + playerCount; pplayer++) {
			if (!player_hasLost(*pplayer)) {
				dealerPlays = true;
				break;
			}
		}
		unsigned int dealerValue = 0;
		if (dealerPlays) {
			dealerValue = player_playAsDealer(dealer, deck);
		}
		for (Player** pplayer = players; pplayer < players + playerCount; pplayer++) {
			player_gameOver(*pplayer, dealerValue);
		}
	}
}

void BlackjackBackend::playerHit() {
	Player* player = players[currentPlayer];
	player_hit(player, deck);
	if (player_isPlaying(player)) {
	} else {
	}
}

void BlackjackBackend::playerStand() {
	Player* player = players[currentPlayer];
	player_stand(player);
	if (player_isPlaying(player)) {
	} else {
	}
}

void BlackjackBackend::playerDouble() {
	Player* player = players[currentPlayer];
	player_double(player, deck);
	if (player_isPlaying(player)) {
	} else {
	}
}

void BlackjackBackend::playerSplit() {
	if (player_split(players[currentPlayer], deck)) {
	}
}

void BlackjackBackend::playerSurrender() {
	Player* player = players[currentPlayer];
	player_surrender(player);
	if (player_isPlaying(player)) {
	} else {
	}
}
