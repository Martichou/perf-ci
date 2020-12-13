table! {
    bench_stat_values (id) {
        id -> Int4,
        label -> Varchar,
        mean -> Float8,
        median -> Float8,
        slope -> Float8,
        commit_hash -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    bench_stats (commit_hash) {
        branch -> Varchar,
        commit_hash -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(bench_stat_values -> bench_stats (commit_hash));

allow_tables_to_appear_in_same_query!(bench_stat_values, bench_stats,);
