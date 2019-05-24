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

#include "standingsmodel.h"

int StandingsModel::columnCount(const QModelIndex &parent) const {
	return 3;
}

void StandingsModel::emitReset() {
	beginResetModel();
	endResetModel();
}

int StandingsModel::rowCount(const QModelIndex &parent) const {
	return pCount + 1;
}

QVariant StandingsModel::data(const QModelIndex &index, int role) const {
	if (index.row() == 0) {
		switch (role) {
			case PName:
				return QVariant("Player");
			case PBalance:
				return QVariant("Balance");
			case PStanding:
				return QVariant("Standing");
		}
	}
	if (!leaderboard) {
		return QVariant();
	}
	int row = index.row() - 1;
	Player* player = leaderboard[row];
	/*switch (role) {
		case PName:
			char* name = player_getName(player);
			QVariant ret = QVariant(QString(name));
			rust_freestr(name);
			return ret;
		case PBal:
			return QVariant(QString("%1").arg(player_getBalance(player)));
		case PStanding:
			return QVariant(QString("%1").arg(player_getStanding(player)));
	}*/
	return QVariant();
}

QHash<int, QByteArray> StandingsModel::roleNames() const {
	QHash<int, QByteArray> names;
	names[PName] = "playerName";
	names[PBalance] = "playerBalance";
	names[PStanding] = "playerStanding";
	return names;
}

/*void StandingsModel::loadStandings(PlayerWrapper* wrapper) {
	leaderboard = wrapper->getStandings();
	pCount = wrapper->getPlayerCount();
	delete wrapper;
	emitReset();
}*/
