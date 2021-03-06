use crate::messages::{UUID};
use crate::lapin::{
    options::*,
};
use futures::{Future, Stream};
use crate::messages::message_handler::MessageHandler;
use lapin_futures::Queue;
use lapin::message::Delivery;
use crate::lapin::types::FieldTable;
use crate::models::OrganizationRepository;
use std::error::Error as StdError;

lazy_static! {
    static ref EXCHANGE: String = std::env::var("EXCHANGE").expect("Missing exh");
}
static QUEUE: &str = "evaluations- organization listener";

#[derive(Clone)]
pub struct MessageListener {
    handler: MessageHandler,
    repo: OrganizationRepository,
    queue: Queue,
}

impl MessageListener {
    pub fn new(url: &str, repo: OrganizationRepository) -> Result<Self, lapin::Error> {
        let handler = MessageHandler::new(url, "Message listener")?;
        let queue = handler.channel.queue_declare(QUEUE, QueueDeclareOptions::default(), FieldTable::default()).wait()?;
        handler.channel.queue_bind(queue.name().as_str(), &EXCHANGE, "organization.*", QueueBindOptions::default(), FieldTable::default()).wait()?;
        info!("Queue {} created", queue.name());

        Ok(MessageListener { handler, repo, queue })
    }

    pub fn run(&self) {
        actix::spawn(self.send_future());
    }

    fn send_future(&self) -> impl Future<Item=(), Error=()> {
        let ch = self.handler.channel.clone();
        let repo = self.repo.clone();

        self.handler.channel.basic_consume(&self.queue, "evaluation consumer", BasicConsumeOptions::default(), FieldTable::default())
            .and_then(move |stream| {
                stream.for_each(move |message| {
                    debug!("Received message from {}", message.routing_key);
                    if let Err(e) = Self::consume_message(&message, repo.clone()) {
                        error!("{}", e);
                    }
                    ch.basic_ack(message.delivery_tag, false)
                })
            }).map_err(|e| error!("{}", e))
    }

    fn consume_message(message: &Delivery, repo: OrganizationRepository) -> Result<(), String> {
        let payload = std::str::from_utf8(&message.data).map_err(|e| e.description().to_string())?;
        let data: UUID = serde_json::from_str(payload).map_err(|e| e.description().to_string())?;

        match message.routing_key.as_str() {
            "organization.created" => {
                info!("Creating organization with id {}", data.id);
                repo.create(data.id).map_err(|e| e.get_message().to_string()).map(|_| ()) }
            "organization.deleted" => {
                info!("Deleting organization with id {}", data.id);
                repo.delete(data.id).map_err(|e| e.get_message().to_string()).map(|_| ()) }
            key => Err(format!("Unknown message from {}", key))
        }?;

        Ok(())
    }
}

