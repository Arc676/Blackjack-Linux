//Copyright (C) 2018-9 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation (version 3)

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.

//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <http://www.gnu.org/licenses/>.

#ifndef STANDINGSMODEL_H
#define STANDINGSMODEL_H

#include <QString>
#include <QAbstractTableModel>
#include "playerwrapper.h"
#include "blackjack.h"

class StandingsModel: public QAbstractTableModel {
	Q_OBJECT

	int pCount = 0;
	Player** standings = nullptr;

	enum {
		PName = Qt::UserRole,
		PBalance,
		PStanding
	};
public:
	int columnCount(const QModelIndex &parent) const;
	int rowCount(const QModelIndex &parent = QModelIndex()) const ;
	QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const;

	QHash<int, QByteArray> roleNames() const;

	/**
	 * Utility method for requesting a model reset from QML
	 */
	Q_INVOKABLE void emitReset();

	/**
	 * Loads a new standings state
	 * @param wrapper A wrapper containing the player standings
	 */
	Q_INVOKABLE void loadStandings(PlayerWrapper* wrapper);
};

#endif
