-- Create the different tables in the database
CREATE TABLE node (
    id BIGINT PRIMARY KEY NOT NULL,
    lat FLOAT NOT NULL,
    lon FLOAT NOT NULL,
    version INT NOT NULL,
    timestamp VARCHAR(50) NOT NULL,
    changeset BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    [user] VARCHAR(50) NOT NULL
);

CREATE TABLE way (
    id BIGINT PRIMARY KEY NOT NULL,
    version INT NOT NULL,
    timestamp VARCHAR(50) NOT NULL,
    changeset BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    [user] VARCHAR(50) NOT NULL
);

CREATE TABLE way_nodes (
    way_id BIGINT NOT NULL,
    ref_id BIGINT NOT NULL,
    FOREIGN KEY (way_id) REFERENCES way(id),
    FOREIGN KEY (ref_id) REFERENCES node(id),
    PRIMARY KEY (way_id, ref_id)
);

CREATE TABLE relation (
    id BIGINT PRIMARY KEY NOT NULL,
    version INT NOT NULL,
    timestamp VARCHAR(50) NOT NULL,
    changeset BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    [user] VARCHAR(50) NOT NULL
);

CREATE TABLE member (
    id BIGINT PRIMARY KEY NOT NULL,
    relation_id BIGINT NOT NULL,
    node_id BIGINT NULL,
    way_id BIGINT NULL,
    relation_ref_id BIGINT NULL,
    member_type VARCHAR(50) NOT NULL,
    role VARCHAR(50) NOT NULL,

    computed_node_id AS (CASE WHEN node_id = -1 THEN NULL ELSE node_id END) PERSISTED,
    computed_way_id AS (CASE WHEN way_id = -1 THEN NULL ELSE way_id END) PERSISTED,
    computed_relation_ref_id AS (CASE WHEN relation_ref_id = -1 THEN NULL ELSE relation_ref_id END) PERSISTED,

    FOREIGN KEY (relation_id) REFERENCES relation(id),
    FOREIGN KEY (computed_node_id) REFERENCES node(id),
    FOREIGN KEY (computed_way_id) REFERENCES way(id),
    FOREIGN KEY (computed_relation_ref_id) REFERENCES relation(id),

    CONSTRAINT member_type_check CHECK (
        (member_type = 'node' AND node_id != -1 AND way_id = -1 AND relation_ref_id = -1) OR
        (member_type = 'way' AND way_id != -1 AND node_id = -1 AND relation_ref_id = -1) OR
        (member_type = 'relation' AND relation_ref_id != -1 AND node_id = -1 AND way_id = -1)
    )
);

-- Node Tags Linking Table
CREATE TABLE node_tags (
    node_id BIGINT NOT NULL,
    [key] VARCHAR(50) NOT NULL,
    value VARCHAR(50) NOT NULL,
    FOREIGN KEY (node_id) REFERENCES node(id),
    PRIMARY KEY (node_id, [key])
);

-- Way Tags Linking Table
CREATE TABLE way_tags (
    way_id BIGINT NOT NULL,
    [key] VARCHAR(50) NOT NULL,
    value VARCHAR(50) NOT NULL,
    FOREIGN KEY (way_id) REFERENCES way(id),
    PRIMARY KEY (way_id, [key])
);

-- Relation Tags Linking Table
CREATE TABLE relation_tags (
    relation_id BIGINT NOT NULL,
    [key] VARCHAR(50) NOT NULL,
    value VARCHAR(50) NOT NULL,
    FOREIGN KEY (relation_id) REFERENCES relation(id),
    PRIMARY KEY (relation_id, [key])
);

DELETE FROM [DenmarkMapsDB].[dbo].[member]
DELETE FROM [DenmarkMapsDB].[dbo].[node_tags]
DELETE FROM [DenmarkMapsDB].[dbo].[relation_tags]
DELETE FROM [DenmarkMapsDB].[dbo].[way_tags]
DELETE FROM [DenmarkMapsDB].[dbo].[way_nodes]
DELETE FROM [DenmarkMapsDB].[dbo].[way]
DELETE FROM [DenmarkMapsDB].[dbo].[relation]
DELETE FROM [DenmarkMapsDB].[dbo].[node]
