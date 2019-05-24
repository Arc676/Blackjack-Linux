qrc!(qml_resources,
    "/" {
        "qml/Main.qml",
        "qml/CardView.qml",
        "qml/ConfirmDialog.qml",
        "qml/DefaultHeader.qml",
        "qml/GameView.qml",
        "qml/SetupView.qml",
        "qml/WrappingLabel.qml"
    },
);

pub fn load() {
    qml_resources();
}
