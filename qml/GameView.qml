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

import QtQuick 2.4
import QtQuick.Layouts 1.1
import Ubuntu.Components 1.3
import Ubuntu.Components.Popups 1.3
import QtMultimedia 5.8

import BlackjackBackend 1.0

Page {
	id: gameViewPage
	anchors.fill: parent

	property SetupView setup

	header: DefaultHeader {}

	function playSound(sfx) {
		if (setup.areSFXEnabled) {
			sfx.play()
		}
	}

	function restartGame() {
		PopupUtils.open(confirmRestartNotif, gameViewPage, {})
	}

	function startGame() {
	}

	Component {
		id: confirmRestartNotif

		ConfirmDialog {
			onRestart: {
				startGame()
			}
		}
	}

	Column {
		spacing: margin
		anchors {
			top: header.bottom
			topMargin: margin
			left: parent.left
			leftMargin: margin
			right: parent.right
			rightMargin: margin
			bottom: parent.bottom
			bottomMargin: margin
		}

		Label {
			id: playerLbl
			text: "Player: (none)"
		}

		Row {
			spacing: margin

			Label {
				text: "Wager:"
			}

			TextField {
				id: wagerField
			}

			Button {
				id: betButton
				text: "Bet"
				onClicked: {
					var wager = parseInt(wagerField.text)
				}
			}
		}

		CardView {
			id: dealerHand
		}

		CardView {
			id: playerHand
		}

		Row {
			spacing: margin

			Button {
				id: hitButton
				text: i18n.tr("Hit")
				onClicked: BlackjackBackend.player_hit()
			}

			Button {
				id: standButton
				text: i18n.tr("Stand")
				onClicked: BlackjackBackend.player_stand()
			}

			Button {
				id: doubleButton
				text: i18n.tr("Double")
				onClicked: BlackjackBackend.player_double()
			}

			Button {
				id: splitButton
				text: i18n.tr("Split")
				onClicked: BlackjackBackend.player_split()
			}

			Button {
				id: surrenderButton
				text: i18n.tr("Surrender")
				onClicked: BlackjackBackend.player_surrender()
			}

			Button {
				id: nextButton
				text: i18n.tr("Next")
				onClicked: BlackjackBackend.pass_turn()
			}
		}

		ListView {
			id: playerTable
			width: parent.width
			height: units.gu(30)
			model: StandingsModel {}
			delegate: Rectangle {
				width: parent.width
				height: pName.height

				Text {
					id: pName
					text: playerName
					anchors.left: parent.left
					anchors.leftMargin: margin
				}

				Text {
					id: pBal
					text: playerBalance
					anchors.left: parent.left
					anchors.leftMargin: units.gu(20)
				}

				Text {
					id: pStanding
					text: playerStanding
					anchors.right: parent.right
					anchors.rightMargin: margin
				}
			}
		}
	}
}
