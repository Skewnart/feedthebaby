CREATE TABLE aliment (
    id bigserial NOT NULL,
    "name" varchar(64) NOT NULL,
    quantity int4 NOT NULL,
    expire_at varchar(16) NOT NULL,
    aliment_type varchar(64) NOT NULL,
    CONSTRAINT aliment_name_aliment_type_key UNIQUE (name, aliment_type),
    CONSTRAINT aliment_pkey PRIMARY KEY (id)
);

CREATE TABLE login (
  id bigserial NOT NULL,
  plain_pwd varchar(64) NOT NULL,
  CONSTRAINT login_pkey PRIMARY KEY (id)
);
