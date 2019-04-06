import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.0
import Greeter 1.0

ApplicationWindow {
    id: window

    width: 640
    height: 480

    title: "Ubuntu Touch ❤️ Rust"

    Greeter {
        id: greeter;
        name: "Rust + Ubuntu Touch"
    }

    StackView {
        anchors.fill: parent

        initialItem: Page {
            header: ToolBar {
                RowLayout {
                    anchors.fill: parent

                    Label {
                        text: "Ubuntu Touch ❤️ Rust"
                    }

                    Item { Layout.fillWidth: true }
                }
            }

            ColumnLayout {
                anchors {
                    fill: parent
                    margins: 16
                }

                spacing: 8

                Label {
                    id: label
                    text: "Press the button below!"
                }

                Button {
                    text: "Compute greeting"
                    onClicked: {
                        label.text = greeter.compute_greetings("Hello, ");
                    }
                }

                Item { Layout.fillHeight: true }
            }
        }
    }

    Component.onCompleted: {
        show();
    }
}
