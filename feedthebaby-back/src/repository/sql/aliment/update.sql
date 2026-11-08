UPDATE aliment
    SET
        name = $2,
        quantity = $3,
        expire_at = $4,
        aliment_type = $5
WHERE id = $1
RETURNING id;