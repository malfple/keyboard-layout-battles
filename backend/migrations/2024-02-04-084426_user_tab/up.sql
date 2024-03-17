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

INSERT INTO layout_tab(sequence_id, layout_data, rating, rating_comfort, time_created, time_modified, name, description)
VALUES
    (1, 'qwertyuiopasdfghjkl;''zxcvbnm,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'QWERTY', 'Does this even need any explanation? - Christopher Latham Sholes 1870-ish'),
    (2, ''',.pyfgcrlaoeuidhtns-;qjkxbmwvz', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Dvorak', 'A popular alternative by August Dvorak 1936'),
	(3, 'ql.p'';fudkarenbgsito/zw,hjvcymx', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Arensito (Simplified)', 'Developed by HÃ¥kon Hallingstad in 2001'),
    (4, 'qwfpgjluy;arstdhneio''zxcvbkm,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Colemak', 'Another popular alternative by Shai Coleman 2006. Easy to learn'),
    (5, 'qwfpbjluy;arstgmneio''zxcdvkh,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Colemak-DH', 'A popular mod to Colemak aimed for ortho and col-stag keyboards. 2014'),
    (6, 'qdrwbjfup;ashtgyneoi''zxmcvkl,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Workman', 'Born out of the dissapointment with Colemak. By OJ Bucao 2010'),
	(7, 'qwdfkjurl;asetgynioh''zxcvbpm,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Norman', 'Only 4 key difference from QWERTY!'),
	(8, 'qwdfkyuil;astrghneop''zxcvbjm,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Minimak-12', 'Only 12 key difference from QWERTY!!!'),
	(9, 'wlrbz;qudjshnt,.aeoi''fmvc/gpxky', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Halmak', 'AI designed. Such technology!'),
	(10, 'flhvzqwuoysrntkcdeai;x''bmjpg,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Semimak', 'A keyboard layout made from genkey, an analyzer focusing on same-finger skipgrams'),
	(11, 'flhvz''wuoysrntkcdeai;xjbmqpg,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Semimak-JQ', 'Improved Semimak'),
	(12, 'flhvz''wuoysrntkgdeai;xjbmqpc,./', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Semimak-JQC', 'Upgraded Improved Semimak!'),
	(13, 'trewqpoiuygfdsa;lkjh''bvcxz/.,mn', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'inqwerted', 'The best keyboard layout to have ever been created'),
    (14, 'ypoujkdlcwinea,mhtsr''qz/.:bfgvx', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'MTGAP-30', 'The Keyboard Layout Project. This is the 30 key version'),
	(15, 'qplcjxfou/rntsgyheia;zbmwvkd'',.', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'CTGAP', 'A series of keyboard layouts by Colby. This is the final version'),
	(16, 'wgdfbqluoyrsthkjneai;xcmpvz,.''/', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'APTv3', 'Based on MTGAP and CTGAP'),
	(17, 'wlypkzxou;crstbfneia''jvdgqmh/,.', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Canary', 'AKL Discord Collab! Guess the number of creators!'),
	(18, 'wlypbzfou''crstgmneia;qjvdkxh/,.', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Canary-ortho', 'Canary but for Ortholinear keyboards'),
	(19, 'flhdmvwou,srntkgyaei/xjbzqpc'';.', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Whorf', 'Whorf!'),
	(20, 'xdh.qbfoujrtna;gweis/lkm,''pczyv', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'RTNA', 'Experimental Layout to get lowest SFB% on colemak mods analyzer'),
	(21, 'xldkvzwou;srtnfgyeia/qjmhbpc'',.', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Sertain', 'low-redirect layout'),
	(22, 'pldvxzkou;nrtsgyheia/qjmcwbf'',.', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Nerts', 'Pretty much nrts-haei'),
	(23, 'xldpvzkou;nrtsgyheia/qjmcwbf'',.', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Nerps', 'Pretty much nrts-haei. Also beginner friendly!'),
	(24, 'bldwz''foujnrtsgyhaei,qxmcvkp.-/', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Graphite', 'Much liked nrts-haei'),
	(25, 'bldcvjyou,nrtsgphaei/xqmwzkf'';.', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Gallium', 'Much comfort nrts-haei'),
	(26, 'bldgqjfou,nrtsvkhaei;zxmcwpy''/.', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Maya', 'Much fast nrts-haei'),
	(27, 'vmlcpxfoujstrdy.naei-zkqgwbh'';,', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Sturdy', 'Get sturdy'),
	(28, 'yclmkzfu,''isrtgpneao;qvwdjbh/.x', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'ISRT', 'Very straightforward name'),
	(29, 'ylcmkzfu,''irstwbneao;qjgdvph".x', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'IRST', 'Is this ISRT''s sister?'),
	(30, 'frdpvqjuoysntcb.heai/zxkgwml;'',', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Recurva', 'Beats Semimak in stats, but will it also achieve victory in battle???'),
	(31, 'qhou,glrfbjieaydtsnp;xk".''vmcwz', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'BEAKL-43', 'BEAKL Fam'),
	(32, 'tkvumiajbrwx/fpdgq,s;h.''colnzye', 1500, 1500, UNIX_TIMESTAMP(), UNIX_TIMESTAMP(), 'Vitrimak', 'A popular and a very good layout (trust)');

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
