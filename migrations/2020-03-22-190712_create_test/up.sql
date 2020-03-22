-- Your SQL goes here
CREATE TABLE users (
   id INT(32) NOT NULL AUTO_INCREMENT PRIMARY KEY,
   name VARCHAR(255) NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO users (name) VALUES ("Christoffer");
CREATE PROCEDURE get_user(
    IN user_id INT(11)
)
BEGIN
    SELECT * FROM users WHERE user_id = user_id;
end;
