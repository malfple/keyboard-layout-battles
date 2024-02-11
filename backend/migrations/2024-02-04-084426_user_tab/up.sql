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
VALUES ('admin', '$2b$08$lES0y3GGF9GL0scfeISeC./hkeVkMJ2j5FA9VExa3SPR.DmFVLnLq', 'frdpwqjuoysntcbmheai,zxkgv.l;''/', UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

CREATE TABLE layout_tab(
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(32) NOT NULL,
    layout_data VARCHAR(256) NOT NULL,
    description VARCHAR(256),
    rating INT NOT NULL,
    rating_comfort INT NOT NULL,
    rating_data JSON,
    is_active BOOLEAN NOT NULL,
    time_created BIGINT NOT NULL,
    time_modified BIGINT NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO layout_tab(layout_data, rating, rating_comfort, is_active, time_created, time_modified, name)
VALUES
    ('qwertyuiopasdfghjkl;''zxcvbnm,./', 1500, 1500, TRUE, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'QWERTY'),
    (''',.pyfgcrlaoeuidhtns-;qjkxbmwvz', 1500, 1500, TRUE, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Dvorak'),
    ('qwfpgjluy;arstdhneio''zxcvbkm,./', 1500, 1500, TRUE, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Colemak'),
    ('qwfpbjluy;arstgmneio''zxcdvkh,./', 1500, 1500, TRUE, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Colemak-DH'),
    ('qdrwbjfup;ashtgyneoi''zxmcvkl,./', 1500, 1500, TRUE, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Workman'),
    ('ypoujkdlcwinea,mhtsr''qz/.:bfgvx', 1500, 1500, TRUE, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'MTGAP');
