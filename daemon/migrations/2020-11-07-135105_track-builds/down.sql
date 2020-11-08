DROP TABLE builds;

PRAGMA foreign_keys=off;

CREATE TABLE _packages_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    distro VARCHAR NOT NULL,
    suite VARCHAR NOT NULL,
    architecture VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    built_at DATETIME,
    attestation VARCHAR,
    checksum VARCHAR,
    retries INTEGER NOT NULL,
    next_retry DATETIME,
    CONSTRAINT packages_unique UNIQUE (name, distro, suite, architecture)
);

INSERT INTO _packages_new (id, name, version, status, distro, suite, architecture, url, built_at, attestation, checksum, retries, next_retry)
    SELECT id, name, version, status, distro, suite, architecture, url, built_at, attestation, checksum, retries, next_retry
    FROM packages;

DROP TABLE packages;
ALTER TABLE _packages_new RENAME TO packages;

PRAGMA foreign_keys=on;
