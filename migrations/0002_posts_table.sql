CREATE TABLE posts
(
    id SERIAL PRIMARY KEY,
    title VARCHAR(256) NOT NULL,
    description TEXT,
    user_id INT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);