use rdkafka::config::ClientConfig;
// kafka_consumer.rs
use rdkafka::consumer::{Consumer, StreamConsumer};
use tokio::sync::mpsc;

pub async fn consume_kafka_topic() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "my-consumer-group")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer.subscribe(&["my-topic"]).expect("Failed to subscribe to topic");

    let (cancel_tx, mut cancel_rx) = mpsc::channel::<()>(10);

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Error waiting for Ctrl+C");
        cancel_tx.send(()).await.expect("Error sending cancellation signal");
    });

    let mut last_100_messages = Vec::new();
    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                println!("Received message: {:?}", m);
                last_100_messages.push(m);
                if last_100_messages.len() > 5 {
                    println!("Last 5 messages: {:?}", last_100_messages);
                    last_100_messages.remove(0);
                }
                if cancel_rx.try_recv().is_ok() {
                    break;
                }
            }
        }
    }

    // while let Some(message) = consumer.recv().await {
    //     // Process the Kafka message (e.g., print it)
    //     println!("Received message: {:?}", message);
    //
    //     // Keep track of the last 100 messages
    //     last_100_messages.push(message);
    //     if last_100_messages.len() > 100 {
    //         last_100_messages.remove(0);
    //     }
    //
    //     // Check for cancellation signal
    //     if cancel_rx.try_recv().is_ok() {
    //         break; // Exit gracefully
    //     }
    // }
}
