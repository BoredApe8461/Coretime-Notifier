/// ## Coretime Notifier

#[tokio::main]
async fn main() {
	storage::initialize_db().unwrap();
	tracker::track().await.unwrap();
}
