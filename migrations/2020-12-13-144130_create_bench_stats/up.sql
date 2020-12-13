CREATE TABLE bench_stats (
	branch VARCHAR(255) NOT NULL,
	commit_hash VARCHAR(40) PRIMARY KEY NOT NULL,
	created_at TIMESTAMP NOT NULL
);
CREATE TABLE bench_stat_values (
	id SERIAL PRIMARY KEY,
	label VARCHAR(128) NOT NULL,
	mean FLOAT NOT NULL,
	median FLOAT NOT NULL,
	slope FLOAT NOT NULL,
	commit_hash VARCHAR(40) NOT NULL,
	created_at TIMESTAMP NOT NULL
);
ALTER TABLE bench_stat_values ADD CONSTRAINT commit_hash_fkey FOREIGN KEY (commit_hash) REFERENCES bench_stats (commit_hash) DEFERRABLE;