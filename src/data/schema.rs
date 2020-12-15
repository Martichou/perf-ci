table! {
    bench_stat_values (id) {
        id -> Int4,
        label -> Varchar,
        mean -> Float8,
        median -> Float8,
        slope -> Float8,
        bsid -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    bench_stats (bsid) {
        bsid -> Int4,
        branch -> Varchar,
        commit_hash -> Varchar,
        os -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    filterable_os (os) {
        os -> Varchar,
    }
}

joinable!(bench_stat_values -> bench_stats (bsid));

allow_tables_to_appear_in_same_query!(
    bench_stat_values,
    bench_stats,
    filterable_os,
);
