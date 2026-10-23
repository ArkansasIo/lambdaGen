CREATE DATABASE IF NOT EXISTS lambdagen;
USE lambdagen;

CREATE TABLE IF NOT EXISTS research_nodes (
  id VARCHAR(191) PRIMARY KEY,
  parent_id VARCHAR(191) NULL,
  kind VARCHAR(32) NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX idx_research_parent (parent_id),
  CONSTRAINT fk_research_parent FOREIGN KEY (parent_id) REFERENCES research_nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS experiments (
  id VARCHAR(191) PRIMARY KEY,
  conjecture_id VARCHAR(191) NULL,
  algorithm VARCHAR(255) NOT NULL,
  input_range VARCHAR(255),
  status VARCHAR(32) NOT NULL,
  tested BIGINT UNSIGNED DEFAULT 0,
  passed BIGINT UNSIGNED DEFAULT 0,
  counterexample TEXT,
  proof_status VARCHAR(64) NOT NULL,
  duration_ms BIGINT UNSIGNED DEFAULT 0,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX idx_experiments_conjecture (conjecture_id)
);

CREATE TABLE IF NOT EXISTS experiment_rows (
  experiment_id VARCHAR(191) NOT NULL,
  row_number BIGINT UNSIGNED NOT NULL,
  column_name VARCHAR(191) NOT NULL,
  value_text TEXT,
  value_number DOUBLE NULL,
  value_boolean BOOLEAN NULL,
  PRIMARY KEY (experiment_id, row_number, column_name),
  CONSTRAINT fk_rows_experiment FOREIGN KEY (experiment_id) REFERENCES experiments(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS jobs (
  id VARCHAR(191) PRIMARY KEY,
  experiment_id VARCHAR(191) NULL,
  cron_expression VARCHAR(128) NULL,
  status VARCHAR(32) NOT NULL,
  payload JSON NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX idx_jobs_status (status)
);
