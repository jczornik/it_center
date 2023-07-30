CREATE TABLE messages (
       id uuid DEFAULT uuid_generate_v4 (),
       title VARCHAR NOT NULL,
       body TEXT NOT NULL,
       status VARCHAR(20) NOT NULL,
       sender_id uuid NOT NULL,
       recipient_id uuid NOT NULL,
       PRIMARY KEY (id),
       CONSTRAINT fk_sender FOREIGN KEY(sender_id) REFERENCES users(id),
       CONSTRAINT fk_recipient FOREIGN KEY(recipient_id) REFERENCES users(id)
)
