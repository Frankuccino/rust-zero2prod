// use actix_web::rt::net::TcpListener;
use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
  // Arrange
  // spawn_app().await.expect("Failed to spawn our app.");
  let address = spawn_app();

  // We need to bring in `reqwest`
  // to perform HTTP requests against our application.
  let client = reqwest::Client::new();
  // Act
  let response = client
      .get(&format!("{}/health_check", &address))
      .send()
      .await
      .expect("Failed to execute request.");

  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
 
}

// Launch our application in the background ~somehow~
// async fn spawn_app() -> std::io::Result<()> {
fn spawn_app() -> String {
  // zero2prod::run()?.await

  let listener = TcpListener::bind("127.0.0.1:0").expect("Failted to bind random port"); 


  let port = listener.local_addr().unwrap().port();
  let server = zero2prod::run(listener).expect("Failed to bind address");

  // Launc the server as a background task
  // tokio::spawn returns a handle to the spawned future,
  // but we have no use for it here, hence the non-binding let
  let _ = actix_web::rt::spawn(server);
  format!("http://127.0.0.1:{}", port)

}