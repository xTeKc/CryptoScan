mod blockchains;
use blockchains::{
    binance,
    cardano,
    ethereum,
    fantom,
    harmony,
    huobi,
    polygon,
    solana,
    xdai
};
use cpp::cpp;

cpp!{{
    #include <iostream>
}}

//UI
#ifndef CRYPDATRSBRCWTK_H
#define CRYPDATRSBRCWTK_H

#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QGroupBox>
#include <QtWidgets/QLabel>
#include <QtWidgets/QLineEdit>
#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QRadioButton>
#include <QtWidgets/QScrollArea>
#include <QtWidgets/QStatusBar>
#include <QtWidgets/QTextBrowser>
#include <QtWidgets/QToolButton>
#include <QtWidgets/QWidget>

QT_BEGIN_NAMESPACE

class Ui_MainWindow
{
public:
    QWidget *centralwidget;
    QToolButton *toolButton;
    QTextBrowser *textBrowser;
    QScrollArea *scrollArea;
    QWidget *scrollAreaWidgetContents;
    QTextBrowser *textBrowser_2;
    QLabel *label;
    QGroupBox *groupBox;
    QGroupBox *groupBox_2;
    QLineEdit *lineEdit;
    QPushButton *pushButton;
    QRadioButton *radioButton;
    QScrollArea *scrollArea_2;
    QWidget *scrollAreaWidgetContents_2;
    QRadioButton *radioButton_2;
    QRadioButton *radioButton_3;
    QRadioButton *radioButton_4;
    QRadioButton *radioButton_5;
    QRadioButton *radioButton_6;
    QRadioButton *radioButton_7;
    QRadioButton *radioButton_8;
    QRadioButton *radioButton_9;
    QRadioButton *radioButton_10;
    QStatusBar *statusbar;

    void setupUi(QMainWindow *MainWindow)
    {
        if (MainWindow->objectName().isEmpty())
            MainWindow->setObjectName(QString::fromUtf8("MainWindow"));
        MainWindow->resize(1265, 868);
        QFont font;
        font.setPointSize(14);
        font.setBold(true);
        font.setWeight(75);
        MainWindow->setFont(font);
        centralwidget = new QWidget(MainWindow);
        centralwidget->setObjectName(QString::fromUtf8("centralwidget"));
        toolButton = new QToolButton(centralwidget);
        toolButton->setObjectName(QString::fromUtf8("toolButton"));
        toolButton->setGeometry(QRect(30, 20, 271, 41));
        QFont font1;
        font1.setPointSize(16);
        font1.setBold(true);
        font1.setWeight(75);
        toolButton->setFont(font1);
        textBrowser = new QTextBrowser(centralwidget);
        textBrowser->setObjectName(QString::fromUtf8("textBrowser"));
        textBrowser->setGeometry(QRect(30, 250, 1211, 581));
        scrollArea = new QScrollArea(centralwidget);
        scrollArea->setObjectName(QString::fromUtf8("scrollArea"));
        scrollArea->setGeometry(QRect(30, 250, 1211, 581));
        scrollArea->setWidgetResizable(true);
        scrollAreaWidgetContents = new QWidget();
        scrollAreaWidgetContents->setObjectName(QString::fromUtf8("scrollAreaWidgetContents"));
        scrollAreaWidgetContents->setGeometry(QRect(0, 0, 1207, 577));
        scrollArea->setWidget(scrollAreaWidgetContents);
        textBrowser_2 = new QTextBrowser(centralwidget);
        textBrowser_2->setObjectName(QString::fromUtf8("textBrowser_2"));
        textBrowser_2->setGeometry(QRect(530, 170, 461, 51));
        label = new QLabel(centralwidget);
        label->setObjectName(QString::fromUtf8("label"));
        label->setGeometry(QRect(350, 170, 171, 51));
        label->setFrameShape(QFrame::NoFrame);
        label->setFrameShadow(QFrame::Plain);
        label->setLineWidth(5);
        groupBox = new QGroupBox(centralwidget);
        groupBox->setObjectName(QString::fromUtf8("groupBox"));
        groupBox->setGeometry(QRect(320, 150, 741, 91));
        groupBox_2 = new QGroupBox(centralwidget);
        groupBox_2->setObjectName(QString::fromUtf8("groupBox_2"));
        groupBox_2->setGeometry(QRect(320, 50, 741, 91));
        lineEdit = new QLineEdit(groupBox_2);
        lineEdit->setObjectName(QString::fromUtf8("lineEdit"));
        lineEdit->setGeometry(QRect(210, 20, 461, 51));
        pushButton = new QPushButton(groupBox_2);
        pushButton->setObjectName(QString::fromUtf8("pushButton"));
        pushButton->setGeometry(QRect(30, 20, 151, 51));
        radioButton = new QRadioButton(centralwidget);
        radioButton->setObjectName(QString::fromUtf8("radioButton"));
        radioButton->setGeometry(QRect(1090, 130, 141, 22));
        radioButton->setLayoutDirection(Qt::RightToLeft);
        scrollArea_2 = new QScrollArea(centralwidget);
        scrollArea_2->setObjectName(QString::fromUtf8("scrollArea_2"));
        scrollArea_2->setGeometry(QRect(30, 70, 271, 171));
        scrollArea_2->setWidgetResizable(true);
        scrollAreaWidgetContents_2 = new QWidget();
        scrollAreaWidgetContents_2->setObjectName(QString::fromUtf8("scrollAreaWidgetContents_2"));
        scrollAreaWidgetContents_2->setGeometry(QRect(0, 0, 267, 167));
        radioButton_2 = new QRadioButton(scrollAreaWidgetContents_2);
        radioButton_2->setObjectName(QString::fromUtf8("radioButton_2"));
        radioButton_2->setGeometry(QRect(10, 10, 151, 22));
        radioButton_3 = new QRadioButton(scrollAreaWidgetContents_2);
        radioButton_3->setObjectName(QString::fromUtf8("radioButton_3"));
        radioButton_3->setGeometry(QRect(10, 40, 151, 22));
        radioButton_4 = new QRadioButton(scrollAreaWidgetContents_2);
        radioButton_4->setObjectName(QString::fromUtf8("radioButton_4"));
        radioButton_4->setGeometry(QRect(10, 70, 151, 22));
        radioButton_5 = new QRadioButton(scrollAreaWidgetContents_2);
        radioButton_5->setObjectName(QString::fromUtf8("radioButton_5"));
        radioButton_5->setGeometry(QRect(10, 100, 151, 22));
        radioButton_6 = new QRadioButton(scrollAreaWidgetContents_2);
        radioButton_6->setObjectName(QString::fromUtf8("radioButton_6"));
        radioButton_6->setGeometry(QRect(10, 130, 151, 22));
        radioButton_7 = new QRadioButton(scrollAreaWidgetContents_2);
        radioButton_7->setObjectName(QString::fromUtf8("radioButton_7"));
        radioButton_7->setGeometry(QRect(160, 10, 101, 22));
        radioButton_7->setLayoutDirection(Qt::RightToLeft);
        radioButton_8 = new QRadioButton(scrollAreaWidgetContents_2);
        radioButton_8->setObjectName(QString::fromUtf8("radioButton_8"));
        radioButton_8->setGeometry(QRect(160, 40, 101, 22));
        radioButton_8->setLayoutDirection(Qt::RightToLeft);
        radioButton_9 = new QRadioButton(scrollAreaWidgetContents_2);
        radioButton_9->setObjectName(QString::fromUtf8("radioButton_9"));
        radioButton_9->setGeometry(QRect(160, 70, 101, 22));
        radioButton_9->setLayoutDirection(Qt::RightToLeft);
        radioButton_10 = new QRadioButton(scrollAreaWidgetContents_2);
        radioButton_10->setObjectName(QString::fromUtf8("radioButton_10"));
        radioButton_10->setGeometry(QRect(160, 100, 101, 22));
        radioButton_10->setLayoutDirection(Qt::RightToLeft);
        scrollArea_2->setWidget(scrollAreaWidgetContents_2);
        MainWindow->setCentralWidget(centralwidget);
        groupBox_2->raise();
        groupBox->raise();
        scrollArea->raise();
        toolButton->raise();
        textBrowser->raise();
        textBrowser_2->raise();
        label->raise();
        radioButton->raise();
        scrollArea_2->raise();
        statusbar = new QStatusBar(MainWindow);
        statusbar->setObjectName(QString::fromUtf8("statusbar"));
        MainWindow->setStatusBar(statusbar);

        retranslateUi(MainWindow);

        QMetaObject::connectSlotsByName(MainWindow);
    } // setupUi

    void retranslateUi(QMainWindow *MainWindow)
    {
        MainWindow->setWindowTitle(QCoreApplication::translate("MainWindow", "CrypDatRs", nullptr));
        toolButton->setText(QCoreApplication::translate("MainWindow", "Blockchains", nullptr));
        textBrowser_2->setPlaceholderText(QString());
        label->setText(QCoreApplication::translate("MainWindow", "Contract Address", nullptr));
        groupBox->setTitle(QString());
        groupBox_2->setTitle(QString());
        lineEdit->setText(QString());
        lineEdit->setPlaceholderText(QCoreApplication::translate("MainWindow", "                Enter Crypto Name or Symbol", nullptr));
        pushButton->setText(QCoreApplication::translate("MainWindow", "Search", nullptr));
        radioButton->setText(QCoreApplication::translate("MainWindow", "Dark Mode", nullptr));
        radioButton_2->setText(QCoreApplication::translate("MainWindow", "Binance", nullptr));
        radioButton_3->setText(QCoreApplication::translate("MainWindow", "Cardano", nullptr));
        radioButton_4->setText(QCoreApplication::translate("MainWindow", "Ethereum", nullptr));
        radioButton_5->setText(QCoreApplication::translate("MainWindow", "Fantom", nullptr));
        radioButton_6->setText(QCoreApplication::translate("MainWindow", "Harmony", nullptr));
        radioButton_7->setText(QCoreApplication::translate("MainWindow", "Huobi", nullptr));
        radioButton_8->setText(QCoreApplication::translate("MainWindow", "Polygon", nullptr));
        radioButton_9->setText(QCoreApplication::translate("MainWindow", "Solana", nullptr));
        radioButton_10->setText(QCoreApplication::translate("MainWindow", "xDai", nullptr));
    } // retranslateUi

};

namespace Ui {
    class MainWindow: public Ui_MainWindow {};
} // namespace Ui

QT_END_NAMESPACE

#endif // CRYPDATRSBRCWTK_H
//UI

pub fn main() {
    binance::call_binance_req();
    cardano::call_cardano_req();
    ethereum::call_ethereum_req();
    fantom::call_fantom_req();
    harmony::call_harmony_req();
    huobi::call_huobi_req();
    polygon::call_polygon_req();
    solana::call_solana_req();
    xdai::call_xdai_req();
}



pub fn binance_bc() {
    let bsc_dropdown = //implement UI dropdown selection
}

pub fn cardano_bc() {
    let ada_dropdown = //implement UI dropdown selection
}

pub fn ethereum_bc() {
    let eth_dropdown = //implement UI dropdown selection
}

pub fn fantom_bc() {
    let ftm_dropdown = //imimplement UI dropdown selection
}

pub fn harmony_bc() {
    let hmy_dropdown = //implement UI dropdown selection
}

pub fn huobi_bc() {
    let huo_dropdown = //implement UI dropdown selection
}

pub fn polygon_bc() {
    let poly_dropdown = //implement UI dropdown selection
}

pub fn solana_bc() {
    let sol_dropdown = //implement UI dropdown selection
}

pub fn xdai_bc() {
    let xdai_dropdown = //implement UI dropdown selection
}