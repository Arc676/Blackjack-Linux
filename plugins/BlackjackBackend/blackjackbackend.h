#ifndef BLACKJACKBACKEND_H
#define BLACKJACKBACKEND_H

#include <QObject>

class BlackjackBackend: public QObject {
    Q_OBJECT

public:
    BlackjackBackend();
    ~BlackjackBackend() = default;

    Q_INVOKABLE void speak();
};

#endif
