#include <QtQml>
#include <QtQml/QQmlContext>

#include "plugin.h"
#include "blackjackbackend.h"

void BlackjackBackendPlugin::registerTypes(const char *uri) {
    //@uri BlackjackBackend
    qmlRegisterSingletonType<BlackjackBackend>(uri, 1, 0, "BlackjackBackend", [](QQmlEngine*, QJSEngine*) -> QObject* { return new BlackjackBackend; });
}
