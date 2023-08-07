mod message;
mod recipient;
mod sender;

use crate::DbPool;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, get, post, web, HttpResponse, Responder, ResponseError};
use actix_web_httpauth::extractors::basic::BasicAuth;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

pub fn generate_message_scope() -> actix_web::Scope {
    web::scope("messages")
        .service(get_all_messages)
        .service(get_all_messages_with_status)
        .service(send_message)
        .service(ack_message_receive)
}

#[derive(Serialize, Debug)]
struct MessageDTO {
    id: String,
    sender: String,
    title: String,
    message: String,
    status: String,
}

#[derive(strum_macros::Display, Debug, Deserialize)]
enum MessageStatus {
    New,
    Received,
    _Read,
    _Deleted,
}

impl MessageDTO {
    fn from(message: message::Message, sender: sender::Sender) -> Self {
        MessageDTO {
            id: message.id.to_string(),
            sender: sender.name,
            title: message.title,
            message: message.body,
            status: message.status,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NewMessageDTO {
    pub title: String,
    pub body: String,
    pub recipient: String,
}

#[derive(Debug, Display, Error)]
enum ProcessingMessageError {
    #[display(fmt = "Message recipient not found")]
    RecipientNotFound,
    #[display(fmt = "Error while saving message")]
    CannotSaveMessage,
    #[display(fmt = "Error while modifying message status")]
    CannotModifyMesageStatus,
    #[display(fmt = "Internal server error")]
    InternalServerError,
}

impl ResponseError for ProcessingMessageError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::RecipientNotFound => StatusCode::BAD_REQUEST,
            Self::CannotSaveMessage => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CannotModifyMesageStatus => StatusCode::BAD_REQUEST,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/all")]
async fn get_all_messages(
    pool: web::Data<DbPool>,
    auth: BasicAuth,
) -> actix_web::Result<impl Responder> {
    let messages: Vec<MessageDTO> = web::block(move || {
        let mut conn = pool
            .get()
            .expect("Should be able to obtain db connection from pool");
        message::select_all_messages(&mut conn, auth.user_id())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?
    .into_iter()
    .map(|(message, sender)| MessageDTO::from(message, sender))
    .collect();

    Ok(HttpResponse::Ok().json(messages))
}

#[get("/filter/{status}")]
async fn get_all_messages_with_status(
    status: web::Path<MessageStatus>,
    pool: web::Data<DbPool>,
    auth: BasicAuth,
) -> actix_web::Result<impl Responder> {
    let status = status.into_inner();
    let messages: Vec<MessageDTO> = web::block(move || {
        let mut conn = pool
            .get()
            .expect("Should be able to obtain db connection from pool");
        message::select_all_messages_with_status(
            &mut conn,
            auth.user_id(),
            status.to_string().as_str(),
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)?
    .into_iter()
    .map(|(message, sender)| MessageDTO::from(message, sender))
    .collect();

    Ok(HttpResponse::Ok().json(messages))
}

#[post("/ack/received/{message_id}")]
async fn ack_message_receive(
    message_id: web::Path<String>,
    pool: web::Data<DbPool>,
    auth: BasicAuth,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool
        .get()
        .expect("Should be able to obtain db connection from pool");

    message::change_message_status(
        &mut conn,
        auth.user_id(),
        message_id.as_str(),
        MessageStatus::Received.to_string().as_str(),
    )
    .map_err(|_| ProcessingMessageError::CannotModifyMesageStatus)?;

    Ok(HttpResponse::Ok())
}

#[post("/new")]
async fn send_message(
    pool: web::Data<DbPool>,
    auth: BasicAuth,
    message: web::Json<NewMessageDTO>,
) -> actix_web::Result<impl Responder> {
    web::block(move || {
        let mut conn = pool
            .get()
            .expect("Should be able to obtain db connection from pool");

        let sender = sender::get_sender_by_name(auth.user_id(), &mut conn)
            .map_err(|_| ProcessingMessageError::InternalServerError)?;
        let recipient = recipient::get_recipient_by_name(&message.recipient, &mut conn)
            .map_err(|_| ProcessingMessageError::RecipientNotFound)?;
        let to_save = message::NewMessage {
            title: message.title.clone(),
            body: message.body.clone(),
            status: MessageStatus::New.to_string(),
            sender_id: sender.id,
            recipient_id: recipient.id,
        };
        message::create_new_message(to_save, &mut conn)
            .map_err(|_| ProcessingMessageError::CannotSaveMessage)?;
        Ok(())
    })
    .await
    .unwrap_or(Err(ProcessingMessageError::InternalServerError))?;

    Ok(HttpResponse::Created())
}
