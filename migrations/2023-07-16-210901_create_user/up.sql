CREATE TABLE users (
       id uuid DEFAULT uuid_generate_v4 (),
       name VARCHAR NOT NULL,
       surename VARCHAR NOT NULL,
       email VARCHAR NOT NULL,
       rule VARCHAR NOT NULL,
       password VARCHAR,
       PRIMARY KEY (id)
)
