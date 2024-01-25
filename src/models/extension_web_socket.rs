#[derive(Clone)]
pub struct ExtensionWebSocket {
    pub tx: barrage::Sender<String>,
    pub rx: barrage::Receiver<String>,
}
