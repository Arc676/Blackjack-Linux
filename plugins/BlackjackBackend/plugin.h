#ifndef BLACKJACKBACKENDPLUGIN_H
#define BLACKJACKBACKENDPLUGIN_H

#include <QQmlExtensionPlugin>

class BlackjackBackendPlugin : public QQmlExtensionPlugin {
    Q_OBJECT
    Q_PLUGIN_METADATA(IID "org.qt-project.Qt.QQmlExtensionInterface")

public:
    void registerTypes(const char *uri);
};

#endif
