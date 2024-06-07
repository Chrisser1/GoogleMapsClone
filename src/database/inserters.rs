use sqlx::{QueryBuilder, SqlitePool};

use crate::{node::Node, way::Way};

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

pub async fn insert_way_data(sqlite_pool: &SqlitePool, ways: Vec<Way>) -> Result<(), sqlx::Error> {
    // SQLite's max number of variables per statement
    const SQLITE_MAX_VARIABLE_NUMBER: usize = 999;
    let way_field_count = 6; // Number of fields per way
    let way_node_field_count = 2; // Number of fields per way_node
    let tag_field_count = 3;  // Number of fields per tag (way_id, key, value)

    // Calculate max ways and tags per batch
    let max_ways_per_batch = SQLITE_MAX_VARIABLE_NUMBER / way_field_count;
    let max_way_nodes_per_batch = SQLITE_MAX_VARIABLE_NUMBER / way_node_field_count;
    let max_tags_per_batch = SQLITE_MAX_VARIABLE_NUMBER / tag_field_count;

    // Ensure we do not exceed the batch size of 4000
    let way_batch_size = max_ways_per_batch.min(4000);
    let way_node_batch_size = max_way_nodes_per_batch.min(4000);
    let tag_batch_size = max_tags_per_batch.min(4000);

    // Insert ways in batches
    for chunk in ways.chunks(way_batch_size) {
        let mut query_builder = QueryBuilder::new(
            "INSERT OR IGNORE INTO way (id, version, timestamp, changeset, uid, [user]) "
        );

        query_builder.push_values(chunk, |mut b, way| {
            b.push_bind(way.id)
                .push_bind(way.version)
                .push_bind(&way.timestamp)
                .push_bind(way.changeset)
                .push_bind(way.uid)
                .push_bind(&way.user);
        });

        let query = query_builder.build();
        query.execute(sqlite_pool).await?;
    }

    // Insert way_nodes in batches
    for chunk in ways.chunks(way_batch_size) {
        let way_nodes = Way::extract_way_node_refs(&chunk);

        for tag_chunk in way_nodes.chunks(way_node_batch_size) {
            let mut way_node_query_builder = QueryBuilder::new(
                "INSERT OR IGNORE INTO way_nodes (way_id, ref_id) "
            );
            way_node_query_builder.push_values(tag_chunk, |mut b, (way_id, ref_id)| {
                b.push_bind(way_id)
                .push_bind(ref_id);
            });

            let way_node_query = way_node_query_builder.build();
            way_node_query.execute(sqlite_pool).await?;
        }
    }

    // Insert way tags in batches
    for chunk in ways.chunks(way_batch_size) {
        let mut tags: Vec<(i64, &str, &str)> = Vec::new();

        for way in chunk {
            for tag in &way.tags {
                tags.push((way.id, &tag.key, &tag.value));
            }
        }

        for tag_chunk in tags.chunks(tag_batch_size) {
            let mut tag_query_builder = QueryBuilder::new(
                "INSERT OR IGNORE INTO way_tags (way_id, [key], value) "
            );

            tag_query_builder.push_values(tag_chunk, |mut b, (way_id, key, value)| {
                b.push_bind(way_id)
                    .push_bind(key)
                    .push_bind(value);
            });

            let tag_query = tag_query_builder.build();
            tag_query.execute(sqlite_pool).await?;
        }
    }

    Ok(())
}
