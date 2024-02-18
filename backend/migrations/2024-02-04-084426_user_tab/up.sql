-- Your SQL goes here

CREATE TABLE user_tab(
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    username VARCHAR(32) NOT NULL,
    password VARCHAR(64) NOT NULL,
    layout_data VARCHAR(64) NOT NULL,
    time_created BIGINT NOT NULL,
    time_modified BIGINT NOT NULL,
    PRIMARY KEY(id),
    UNIQUE INDEX unq_username(username)
);

INSERT INTO user_tab(username, password, layout_data, time_created, time_modified)
VALUES ('admin', '$2b$08$lES0y3GGF9GL0scfeISeC./hkeVkMJ2j5FA9VExa3SPR.DmFVLnLq', 'frdpwqjuoysntcbmheai,zxkgv.l;''/', UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

CREATE TABLE layout_tab(
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    sequence_id BIGINT UNSIGNED,
    name VARCHAR(32) NOT NULL,
    layout_data VARCHAR(64) NOT NULL,
    description VARCHAR(256),
    rating INT NOT NULL,
    rating_comfort INT NOT NULL,
    rating_data JSON,
    time_created BIGINT NOT NULL,
    time_modified BIGINT NOT NULL,
    PRIMARY KEY(id),
    UNIQUE INDEX unq_sequence_id(sequence_id)
);

INSERT INTO layout_tab(sequence_id, layout_data, rating, rating_comfort, time_created, time_modified, name)
VALUES
    (1, 'qwertyuiopasdfghjkl;''zxcvbnm,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'QWERTY'),
    (2, ''',.pyfgcrlaoeuidhtns-;qjkxbmwvz', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Dvorak'),
    (3, 'qwfpgjluy;arstdhneio''zxcvbkm,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Colemak'),
    (4, 'qwfpbjluy;arstgmneio''zxcdvkh,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Colemak-DH'),
    (5, 'qdrwbjfup;ashtgyneoi''zxmcvkl,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Workman'),
    (6, 'ypoujkdlcwinea,mhtsr''qz/.:bfgvx', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'MTGAP');

CREATE TABLE battle_tab(
    id VARCHAR(32) NOT NULL,
    layout_id_1 BIGINT UNSIGNED NOT NULL,
    layout_id_2 BIGINT UNSIGNED NOT NULL,
    base_layout_data VARCHAR(64) NOT NULL,
    user_id_typer BIGINT UNSIGNED,
    content_data JSON NOT NULL,
    is_personal BOOLEAN NOT NULL,
    time_created BIGINT NOT NULL,
    time_modified BIGINT NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE battle_history_tab(
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    layout_id_1 BIGINT UNSIGNED NOT NULL,
    layout_id_2 BIGINT UNSIGNED NOT NULL,
    base_layout_data VARCHAR(64) NOT NULL,
    user_id_typer BIGINT UNSIGNED,
    layout_1_rating INT NOT NULL,
    layout_2_rating INT NOT NULL,
    rating_1_gain INT NOT NULL,
    rating_2_gain INT NOT NULL,
    result_data JSON NOT NULL,
    is_personal BOOLEAN NOT NULL,
    time_created BIGINT NOT NULL,
    PRIMARY KEY(id)
);
