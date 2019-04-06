#[macro_use]
extern crate cstr;
#[macro_use]
extern crate cpp;
#[macro_use]
extern crate qmetaobject;

use qmetaobject::*;

mod qrc;

#[derive(QObject,Default)]
struct Greeter {
    base : qt_base_class!(trait QObject),
    name : qt_property!(QString; NOTIFY name_changed),
    name_changed : qt_signal!(),
    compute_greetings : qt_method!(fn compute_greetings(&self, verb : String) -> QString {
        return (verb + " " + &self.name.to_string()).into()
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
    qml_register_type::<Greeter>(cstr!("Greeter"), 1, 0, cstr!("Greeter"));
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/qml/Main.qml".into());
    engine.exec();
}
