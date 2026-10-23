# Database and Spreadsheet Layer

LambdaGen treats the MySQL database and spreadsheet data table as two views of the same research data model.

## Data flow

```text
Lambda Program
    -> Input API
    -> Job Queue / Cron
    -> Research Tree
    -> Parallel / Cluster Execution
    -> Experiment Result
    -> MySQL
    -> DataTable
    -> CSV / Excel-compatible export / API output
```

## Spreadsheet model

`DataTable` provides named columns and typed cells. It supports row validation and CSV output. CSV is directly consumable by Excel and other spreadsheet programs; an HTML-table representation is also provided for browser/API views.

## MySQL tables

- `research_nodes` — tree, branches, modules, algorithms and research hierarchy metadata.
- `experiments` — execution metadata and mathematical results.
- `experiment_rows` — tabular experiment observations suitable for spreadsheet export.
- `jobs` — queued and scheduled experiment work.

The schema intentionally separates experiment metadata from experiment rows so a large result set can be paged, queried, aggregated, and exported without storing the entire spreadsheet as one opaque field.
