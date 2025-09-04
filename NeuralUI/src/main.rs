use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("https://raw.githubusercontent.com/temidaradev/NeuralRust/refs/heads/master/NeuralUI/qml/main.qml?token=GHSAT0AAAAAADKQKRWB5ZVKJQHZLVDAB3P22FZUFXQ"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
