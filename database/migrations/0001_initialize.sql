CREATE TABLE IF NOT EXISTS temperature (
    timestamp INTEGER PRIMARY KEY,
    value FLOAT NOT NULL
);

CREATE TABLE IF NOT EXISTS battery (
    timestamp INTEGER PRIMARY KEY,
    value FLOAT NOT NULL
);

CREATE TABLE IF NOT EXISTS humidity (
    timestamp INTEGER PRIMARY KEY,
    value FLOAT NOT NULL
);

CREATE TABLE IF NOT EXISTS position (
    timestamp INTEGER PRIMARY KEY,
    latitude FLOAT NOT NULL,
    longitude FLOAT NOT NULL
);
