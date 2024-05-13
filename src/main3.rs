// mod main2;
// mod kafka_consumer;
//
// use rdkafka::client::ClientContext;
// use rdkafka::consumer::{Consumer, ConsumerContext};
// use rdkafka::error::KafkaError;
// use rdkafka::message::{Message};
// use rdkafka::topic_partition_list::TopicPartitionList;
// use tokio::sync::mpsc;
//
// struct KafkaConsumer {
//     consumer: dyn Consumer,
//     topic: String,
//     partition: i32,
//     channel: mpsc::Receiver<()>,
// }
//
// impl KafkaConsumer {
//     async fn new(
//         topic: String,
//         partition: i32,
//         channel: mpsc::Receiver<()>,
//     ) -> Result<Self, KafkaError> {
//         let consumer = Consumer::new(
//             ClientContext::new()
//                 .set("(link unavailable)", "my_group")
//                 .set("bootstrap.servers", "localhost:9092"),
//             TopicPartitionList::new(   ),
//         )?;
//
//         Ok(Self {
//             consumer,
//             topic,
//             partition,
//             channel,
//         })
//     }
//
//     async fn run(&mut self) -> Result<(), KafkaError> {
//         loop {
//             tokio::select! {
//                 msg = self.consumer.recv() => {
//                     match msg {
//                         Message::Message(msg) => {
//                             println!("Received message: {}", msg.payload);
//                         }
//                         Message::Offset(msg) => {
//                             println!("Received offset: {}", msg.offset);
//                         }
//                     }
//                 }
//                 _ = self.channel.recv() => {
//                     println!("Received cancel signal, exiting...");
//                     break;
//                 }
//             }
//         }
//
//         Ok(())
//     }
// }
//
// #[tokio::main]
// async fn main() -> Result<(), KafkaError> {
//     let (tx, rx) = mpsc::channel(1);
//     let mut consumer = KafkaConsumer::new("my_topic".to_string(), 0, rx).await?;
//
//     tokio::spawn(async move {
//         consumer.run().await?;
//     });
//
//     // Send cancel signal after 10 seconds
//     tokio::time::sleep(std::time::Duration::from_secs(10)).await;
//     tx.send(()).await?;
//
//     Ok(())
// }
//
//
// // /*
// //     Function to
// //
// //     create_topics (config_id, topicRequest) -> topicResponse
// //     delete_topics (config_id, topicRequest) -> topicResponse
// //     list_topics (config_id) -> topicResponse
// //
// //  */
// //
// // use stores::{Reduceable, State, Store};
// //
// // use crate::utils::setup_logger;
// //
// // mod utils;
// // mod config;
// // mod model;
// //
// // #[derive(Default, Debug)]
// // struct Counter {
// //     v: u32,
// // }
// //
// // enum Action {
// //     Increment,
// //     Decrement,
// //     Nothing,
// // }
// //
// // impl Reduceable<Action> for Counter {
// //     fn reduce(state: State<Self>, action: &Action) -> State<Self> {
// //         let prev = &*state.lock();
// //         match action {
// //             Action::Increment => {
// //                 return State::new(Counter {
// //                     v: prev.v + 1,
// //                     ..*prev
// //                 });
// //             }
// //             Action::Decrement => {
// //                 return State::new(Counter {
// //                     v: prev.v - 1,
// //                     ..*prev
// //                 });
// //             }
// //             _ => {}
// //         }
// //         state.clone()
// //     }
// // }
// //
// // fn main() {
// //     setup_logger(true, Some("info"));
// //
// //     let mut r = Store::<Counter, Action>::get().lock();
// //     r.watch(|state| {
// //         println!("state changed! {:?}", state.lock());
// //     });
// //     r.dispatch(&Action::Increment);
// //     r.dispatch(&Action::Increment);
// //     r.dispatch(&Action::Increment);
// //     r.dispatch(&Action::Increment);
// //     r.dispatch(&Action::Decrement);
// //
// //     // let config_request = ClientConfigRequest {
// //     //     client_id: "1231lkjdsf".to_string(),
// //     //     bootstrap_servers: "localhost:9092".to_string()
// //     // };
// //     //
// //     // let client_config = config_utils::nv_create_config(&config_request);
// //     // log::info!("Client Config: {:?}", client_config);
// // }
// //
// //
