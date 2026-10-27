UPDATE aliment
    SET quantity = $1
WHERE id = $2
RETURNING id;