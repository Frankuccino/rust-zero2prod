#[actix_web::test]
async fn health_check_works() {
  // Arrange
  // spawn_app().await.expect("Failed to spawn our app.");
  spawn_app();

  // We need to bring in `reqwest`
  // to perform HTTP requests against our application.
  let client = reqwest::Client::new();
  // Act
  let response = client
      .get("http://127.0.0.1:8000/health_check")
      .send()
      .await
      .expect("Failed to execute request.");

  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
 
}

// Launch our application in the background ~somehow~
// async fn spawn_app() -> std::io::Result<()> {
fn spawn_app() {
  // zero2prod::run()?.await
  let server = zero2prod::run().expect("Failed to bind address");

  // Launc the server as a background task
  // tokio::spawn returns a handle to the spawned future,
  // but we have no use for it here, hence the non-binding let
  let _ = actix_web::rt::spawn(server);

}