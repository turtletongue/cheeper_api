use domain::value_objects::UserId;
use errors::Error;

use crate::dto::messages::{
    ListMessagesFromIntervalDto, ListMessagesFromIntervalParams, SendMessageDto,
};
use crate::repositories::{MessagesRepository, UsersRepository};

pub struct MessageService<K: MessagesRepository, V: UsersRepository> {
    messages_repository: K,
    users_repository: V,
}

impl<K: MessagesRepository, V: UsersRepository> MessageService<K, V> {
    pub fn new(messages_repository: K, users_repository: V) -> Self {
        MessageService {
            messages_repository,
            users_repository,
        }
    }

    pub async fn find_from_interval(
        &self,
        params: ListMessagesFromIntervalParams,
        receiver_id: UserId,
    ) -> Result<ListMessagesFromIntervalDto, Error> {
        let messages = self
            .messages_repository
            .messages_from_interval(
                params.sender_id,
                receiver_id,
                params.from_date,
                params.to_date,
            )
            .await?;

        Ok(ListMessagesFromIntervalDto {
            items: messages.into_iter().map(|message| message.into()).collect(),
        })
    }

    pub async fn send_message(&self, dto: SendMessageDto, sender_id: UserId) -> Result<(), Error> {
        let sender = self.users_repository.user_of_id(sender_id).await?;

        if let None = sender {
            return Err(Error::NotFoundError {
                message: "Sender not found",
            });
        }

        let receiver = self.users_repository.user_of_id(dto.receiver_id).await?;

        if let None = receiver {
            return Err(Error::NotFoundError {
                message: "Receiver not found",
            });
        }

        let sender = sender.unwrap();
        let receiver = receiver.unwrap();

        let message_id = self.messages_repository.next_identity().await;
        let message = sender.write_message_to(&receiver, message_id, dto.text)?;
        self.messages_repository.save(message).await?;

        Ok(())
    }
}
