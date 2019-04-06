qrc!(qml_resources,
    "/" {
        "qml/Main.qml"
    },
);

pub fn load() {
    qml_resources();
}
