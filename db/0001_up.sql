CREATE TABLE files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name NVARCHAR(45) NOT NULL,
    user_id INTEGER NOT NULL,
    tags NVARCHAR(1000),
    public BOOLEAN,
    votes BIGINT,
    deleted BOOLEAN,

    CONSTRAINT fk_user_id
        FOREIGN KEY (user_id)
        REFERENCES users (id)
);

CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_name NVARCHAR(255) NOT NULL,
    email NVARCHAR(255) NOT NULL,
    pass NVARCHAR(30) NOT NULL,
    pass_salt NVARCHAR(16) NOT NULL,
    deleted BOOLEAN,
    notify BOOLEAN
);


INSERT INTO users (user_name, email, pass, pass_salt) VALUES ('none', 'none@none.com', 'fdq2ergfds', 'fsadfhfjhgdtyjuthgf');
INSERT INTO users (user_name, email, pass, pass_salt) VALUES ('arnold', 'arnoldsmithson@outlook.com', 'a', 'b');
