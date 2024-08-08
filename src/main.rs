/// ## Coretime Notifier

#[tokio::main]
async fn main() {
    tracker::track().await.unwrap();
}
