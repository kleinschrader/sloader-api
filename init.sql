CREATE DATABASE sloader;
USE sloader;

CREATE TABLE Users (
    UserID BINARY(16),
    FullName VARCHAR(128),
    Username VARCHAR(64),
    Salt VARCHAR(12),
    HashedPassword VARCHAR(100),
    Administrator BOOLEAN
);

INSERT INTO Users VALUES (
    0x0,
    "Administrator",
    "Administrator",
    "saltysalt",
    "$argon2i$v=19$m=4096,t=3,p=1$c2FsdHlzYWx0$aHYuEEaf0a68WsIPg+nWXzLwUk+V1yNF//J8yPqFXHY",
    true
);

CREATE TABLE Targets (
    TargetID BINARY(16),
    NickName VARCHAR(128),
    TargetPath VARCHAR(4096)
);

CREATE TABLE TargetPermissions (
    TargetID BINARY(16),
    UserID BINARY(16),
    Permission TINYINT
);