use sqlx::SqlitePool;


pub async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create tables if they do not exist
    let create_node_table = "
    CREATE TABLE IF NOT EXISTS node (
        id BIGINT PRIMARY KEY NOT NULL,
        lat FLOAT NOT NULL,
        lon FLOAT NOT NULL,
        version INT NOT NULL,
        timestamp VARCHAR(50) NOT NULL,
        changeset BIGINT NOT NULL,
        uid BIGINT NOT NULL,
        [user] VARCHAR(50) NOT NULL
    );";

    let create_way_table = "
    CREATE TABLE IF NOT EXISTS way (
        id BIGINT PRIMARY KEY NOT NULL,
        version INT NOT NULL,
        timestamp VARCHAR(50) NOT NULL,
        changeset BIGINT NOT NULL,
        uid BIGINT NOT NULL,
        [user] VARCHAR(50) NOT NULL
    );";

    let create_way_nodes_table = "
    CREATE TABLE IF NOT EXISTS way_nodes (
        way_id BIGINT NOT NULL,
        ref_id BIGINT NOT NULL,
        FOREIGN KEY (way_id) REFERENCES way(id),
        FOREIGN KEY (ref_id) REFERENCES node(id),
        PRIMARY KEY (way_id, ref_id)
    );";

    let create_relation_table = "
    CREATE TABLE IF NOT EXISTS relation (
        id BIGINT PRIMARY KEY NOT NULL,
        version INT NOT NULL,
        timestamp VARCHAR(50) NOT NULL,
        changeset BIGINT NOT NULL,
        uid BIGINT NOT NULL,
        [user] VARCHAR(50) NOT NULL
    );";

    let create_member_table = "
    CREATE TABLE IF NOT EXISTS member (
        id BIGINT PRIMARY KEY NOT NULL,
        relation_id BIGINT NOT NULL,
        node_id BIGINT NULL,
        way_id BIGINT NULL,
        relation_ref_id BIGINT NULL,
        member_type VARCHAR(50) NOT NULL,
        role VARCHAR(50) NOT NULL,

        FOREIGN KEY (relation_id) REFERENCES relation(id),

        CONSTRAINT member_type_check CHECK (
            (member_type = 'node' AND node_id IS NOT NULL AND way_id IS NULL AND relation_ref_id IS NULL) OR
            (member_type = 'way' AND way_id IS NOT NULL AND node_id IS NULL AND relation_ref_id IS NULL) OR
            (member_type = 'relation' AND relation_ref_id IS NOT NULL AND node_id IS NULL AND way_id IS NULL)
        )
    );";

    let create_node_tags_table = "
    CREATE TABLE IF NOT EXISTS node_tags (
        node_id BIGINT NOT NULL,
        [key] VARCHAR(50) NOT NULL,
        value VARCHAR(50) NOT NULL,
        FOREIGN KEY (node_id) REFERENCES node(id),
        PRIMARY KEY (node_id, [key])
    );";

    let create_way_tags_table = "
    CREATE TABLE IF NOT EXISTS way_tags (
        way_id BIGINT NOT NULL,
        [key] VARCHAR(50) NOT NULL,
        value VARCHAR(50) NOT NULL,
        FOREIGN KEY (way_id) REFERENCES way(id),
        PRIMARY KEY (way_id, [key])
    );";

    let create_relation_tags_table = "
    CREATE TABLE IF NOT EXISTS relation_tags (
        relation_id BIGINT NOT NULL,
        [key] VARCHAR(50) NOT NULL,
        value VARCHAR(50) NOT NULL,
        FOREIGN KEY (relation_id) REFERENCES relation(id),
        PRIMARY KEY (relation_id, [key])
    );";

    // Execute the queries to create tables and print results
    let result = sqlx::query(create_node_table).execute(pool).await;
    println!("Create node table result: {:?}", result);

    let result = sqlx::query(create_way_table).execute(pool).await;
    println!("Create way table result: {:?}", result);

    let result = sqlx::query(create_way_nodes_table).execute(pool).await;
    println!("Create way_nodes table result: {:?}", result);

    let result = sqlx::query(create_relation_table).execute(pool).await;
    println!("Create relation table result: {:?}", result);

    let result = sqlx::query(create_member_table).execute(pool).await;
    println!("Create member table result: {:?}", result);

    let result = sqlx::query(create_node_tags_table).execute(pool).await;
    println!("Create node_tags table result: {:?}", result);

    let result = sqlx::query(create_way_tags_table).execute(pool).await;
    println!("Create way_tags table result: {:?}", result);

    let result = sqlx::query(create_relation_tags_table).execute(pool).await;
    println!("Create relation_tags table result: {:?}", result);

    Ok(())
}
