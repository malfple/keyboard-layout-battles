-- Your SQL goes here

CREATE TABLE user_tab(
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    username VARCHAR(32) NOT NULL,
    password VARCHAR(64) NOT NULL,
    layout_data VARCHAR(255) NOT NULL,
    time_created BIGINT NOT NULL,
    time_modified BIGINT NOT NULL,
    PRIMARY KEY(id),
    UNIQUE INDEX unq_username (username)
);

INSERT INTO user_tab(username, password, layout_data, time_created, time_modified)
VALUES ("admin", "admin", "sntc", UNIX_TIMESTAMP(), UNIX_TIMESTAMP());
