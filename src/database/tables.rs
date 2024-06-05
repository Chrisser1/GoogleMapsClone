use futures::{stream, StreamExt};
use sqlx::{Execute, FromRow, QueryBuilder, SqlitePool};

use crate::node::Node;

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
        FOREIGN KEY (node_id) REFERENCES node(id),
        FOREIGN KEY (way_id) REFERENCES way(id),
        FOREIGN KEY (relation_ref_id) REFERENCES relation(id),

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

pub async fn insert_node_data(sqlite_pool: &SqlitePool, nodes: Vec<Node>) -> Result<(), sqlx::Error> {
    // SQLite's max number of variables per statement
    const SQLITE_MAX_VARIABLE_NUMBER: usize = 999;
    let node_field_count = 8; // Number of fields per node
    let tag_field_count = 3;  // Number of fields per tag (node_id, key, value)

    // Calculate max nodes and tags per batch
    let max_nodes_per_batch = SQLITE_MAX_VARIABLE_NUMBER / node_field_count;
    let max_tags_per_batch = SQLITE_MAX_VARIABLE_NUMBER / tag_field_count;

    // Ensure we do not exceed the batch size of 4000
    let node_batch_size = max_nodes_per_batch.min(4000);
    let tag_batch_size = max_tags_per_batch.min(4000);

    // Insert nodes in batches
    for chunk in nodes.chunks(node_batch_size) {
        let mut query_builder = QueryBuilder::new(
            "INSERT OR IGNORE INTO node (id, lat, lon, version, timestamp, changeset, uid, [user]) "
        );

        query_builder.push_values(chunk, |mut b, node| {
            b.push_bind(node.id)
                .push_bind(node.lat)
                .push_bind(node.lon)
                .push_bind(node.version)
                .push_bind(&node.timestamp)
                .push_bind(node.changeset)
                .push_bind(node.uid)
                .push_bind(&node.user);
        });

        let query = query_builder.build();
        query.execute(sqlite_pool).await?;
    }

    // Insert node tags in batches
    for chunk in nodes.chunks(node_batch_size) {
        let mut tags: Vec<(i64, &str, &str)> = Vec::new();

        for node in chunk {
            for tag in &node.tags {
                tags.push((node.id, &tag.key, &tag.value));
            }
        }

        for tag_chunk in tags.chunks(tag_batch_size) {
            let mut tag_query_builder = QueryBuilder::new(
                "INSERT OR IGNORE INTO node_tags (node_id, [key], value) "
            );

            tag_query_builder.push_values(tag_chunk, |mut b, (node_id, key, value)| {
                b.push_bind(node_id)
                    .push_bind(key)
                    .push_bind(value);
            });

            let tag_query = tag_query_builder.build();
            tag_query.execute(sqlite_pool).await?;
        }
    }

    Ok(())
}


pub async fn fetch_all_nodes_and_tags(sqlite_pool: &SqlitePool) -> Result<Vec<Node>, sqlx::Error> {
    let query = "
        SELECT
            n.id, n.lat, n.lon, n.version, n.timestamp, n.changeset, n.uid, n.[user],
            GROUP_CONCAT(nt.[key] || ':' || nt.value, ',') as tags
        FROM
            node n
        LEFT JOIN
            node_tags nt ON n.id = nt.node_id
        GROUP BY
            n.id
    ";

    let fetched_result = sqlx::query(query)
        .fetch_all(sqlite_pool)
        .await?;

    let mut nodes = Vec::new();

    // Process fetched rows
    for row in fetched_result {
        let node:Node = Node::from_row(&row)?;
        nodes.push(node);
    }

    Ok(nodes)
}
