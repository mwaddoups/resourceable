CREATE TABLE spaceship (
    spaceship_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    spaceship_num_thrusters INT NOT NULL,
    spaceship_name VARCHAR(255) NOT NULL
)
