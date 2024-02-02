#[derive(Clone)]
pub struct ExtensionWebSocketMatch {
    pub tx: barrage::Sender<String>,
    pub rx: barrage::Receiver<String>,
}

#[derive(Clone)]
pub struct ExtensionWebSocketError {
    pub tx: barrage::Sender<String>,
    pub rx: barrage::Receiver<String>,
}
