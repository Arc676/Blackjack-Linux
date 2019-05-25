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

#ifndef BLACKJACKBACKEND_H
#define BLACKJACKBACKEND_H

#include <QObject>
#include "blackjack.h"

class BlackjackBackend: public QObject {
	Q_OBJECT

	Deck* deck;
	Player** players;
	Player* dealer;
	int playerCount = 0, currentPlayer = 0;

public:
	BlackjackBackend();
	~BlackjackBackend() = default;

	void newGame(int deckCount, int playerCount);

	void beginTurn(int wager);

	void passTurn();

	void playerHit();

	void playerStand();

	void playerDouble();

	void playerSplit();

	void playerSurrender();
};

#endif
