CREATE TABLE aliment (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(64) UNIQUE NOT NULL,
    quantity        INTEGER NOT NULL,
    expire_at       VARCHAR(16) NOT NULL,
    aliment_type    VARCHAR(64) NOT NULL
);

CREATE TABLE login (
    id              BIGSERIAL PRIMARY KEY,
    plain_pwd       VARCHAR(64) NOT NULL
);
