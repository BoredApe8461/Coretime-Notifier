/// ## Coretime Notifier

#[tokio::main]
async fn main() {
	// Initialize the API service
	api::rocket().await.launch().await.unwrap();
	tracker::track().await.unwrap();
}
