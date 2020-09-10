CREATE TABLE movies (
    id              INTEGER PRIMARY KEY,
    title           VARCHAR(255) NOT NULL,
    release_date    DATE NOT NULL,
    file_path       VARCHAR(255) NOT NULL
);