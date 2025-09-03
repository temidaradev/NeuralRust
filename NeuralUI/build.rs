use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qt_module("Quick")
        .qml_module(QmlModule::<&str, _> {
            uri: "com.kdab.cxx_qt.demo",
            rust_files: &[],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        .build();
}
