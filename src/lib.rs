use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use nats::asynchronous::Connection;
use nats::asynchronous::Message;
use nats::asynchronous::Options;
use std::sync::Arc;
use tokio::sync::Mutex;

struct NatsClient {
    conn: Arc<Mutex<Connection>>,
}

impl NatsClient {
    async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let options = Options::new().add_root_certificate("path/to/cert.pem");
        let conn = options.connect(url).await?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    async fn send_message(&self, subject: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.conn.lock().await;
        conn.publish(subject, message).await?;
        Ok(())
    }
}

#[no_mangle]
pub extern "system" fn Java_com_example_smscollector_SmsReceiver_sendToNats(
    env: JNIEnv,
    _: JClass,
    message: JString,
) -> jstring {
    let message: String = env.get_string(message).expect("Invalid message string").into();
    let nats_url = "nats://localhost:4222";
    let subject = "sms.events";

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let client = NatsClient::new(nats_url).await.expect("Failed to connect to NATS");
        client.send_message(subject, &message).await.expect("Failed to send message");
    });

    env.new_string("Message sent to NATS").unwrap().into_inner()
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
