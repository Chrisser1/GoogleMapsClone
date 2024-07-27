use sqlx::{FromRow, SqlitePool};

use crate::{node::Node, relation::Relation, way::Way};

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
        let node: Node = Node::from_row(&row)?;
        nodes.push(node);
    }

    Ok(nodes)
}

pub async fn fetch_all_ways_and_tags(sqlite_pool: &SqlitePool) -> Result<Vec<Way>, sqlx::Error> {
    let query = "
        SELECT
            w.id, w.version, w.timestamp, w.changeset, w.uid, w.[user],
            node_refs.node_refs,
            way_tags.tags
        FROM
            way w
        LEFT JOIN (
            SELECT
                wn.way_id,
                GROUP_CONCAT(wn.ref_id, ',') as node_refs
            FROM
                way_nodes wn
            GROUP BY
                wn.way_id
        ) as node_refs ON w.id = node_refs.way_id
        LEFT JOIN (
            SELECT
                wt.way_id,
                GROUP_CONCAT(wt.[key] || ':' || wt.value, ',') as tags
            FROM
                way_tags wt
            GROUP BY
                wt.way_id
        ) as way_tags ON w.id = way_tags.way_id
    ";

    let fetched_result = sqlx::query(query)
        .fetch_all(sqlite_pool)
        .await?;

    let mut ways = Vec::new();

    // Process fetched rows
    for row in fetched_result {
        let way: Way = Way::from_row(&row)?;
        ways.push(way);
    }

    Ok(ways)
}

pub async fn fetch_all_relations_and_tags(sqlite_pool: &SqlitePool) -> Result<Vec<Relation>, sqlx::Error> {
    let query = "
        SELECT
            r.id, r.version, r.timestamp, r.changeset, r.uid, r.[user],
            relation_tags.tags,
            member.members
        FROM
            relation r
        LEFT JOIN (
            SELECT
                rt.relation_id,
                GROUP_CONCAT(rt.[key] || ':' || rt.value, ',') as tags
            FROM
                relation_tags rt
            GROUP BY
                rt.relation_id
        ) as relation_tags ON r.id = relation_tags.relation_id
        LEFT JOIN (
            SELECT
                m.relation_id,
                GROUP_CONCAT(m.id || ':' || IFNULL(m.node_id, '') || ':' || IFNULL(m.way_id, '') || ':' || IFNULL(m.relation_ref_id, '') || ':' || m.member_type || ':' || m.role, ',') as members
            FROM
                member m
            GROUP BY
                m.relation_id
        ) as member ON r.id = member.relation_id
    ";

    let fetched_result = sqlx::query(query)
        .fetch_all(sqlite_pool)
        .await?;

    let mut relations = Vec::new();

    // Process fetched rows
    for row in fetched_result {
        let relation: Relation = Relation::from_row(&row)?;
        relations.push(relation);
    }

    Ok(relations)
}
