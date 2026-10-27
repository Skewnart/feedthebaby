INSERT INTO aliment(name, quantity, expire_at, aliment_type)
VALUES ($1, $2, $3, $4)
RETURNING id;
