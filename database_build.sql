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
    node_id BIGINT,
    way_id BIGINT,
    relation_ref_id BIGINT,
    member_type VARCHAR(50) NOT NULL,
    role VARCHAR(50) NOT NULL,
    FOREIGN KEY (relation_id) REFERENCES relation(id),
    FOREIGN KEY (node_id) REFERENCES node(id),
    FOREIGN KEY (way_id) REFERENCES way(id),
    FOREIGN KEY (relation_ref_id) REFERENCES relation(id),
    CONSTRAINT member_type_check CHECK (
        (member_type = 'node' AND node_id IS NOT NULL AND way_id IS NULL AND relation_ref_id IS NULL) OR
        (member_type = 'way' AND way_id IS NOT NULL AND node_id IS NULL AND relation_ref_id IS NULL) OR
        (member_type = 'relation' AND relation_ref_id IS NOT NULL AND node_id IS NULL AND way_id IS NULL)
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
