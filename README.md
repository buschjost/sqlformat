# sqlformat 

This is a tiny CLI tool to format SQL queries in your terminal or your scripts. All heavy lifting is done by the [sqlformat-rs](https://github.com/shssoichiro/sqlformat-rs) library. 

```
sqlformat 0.0.1-development

Simple SQL formatter. Reads from stdin and writes to stdout.

https://github.com/buschjost/sqlformat

USAGE:
    sqlformat

EXAMPLES:
    echo 'SELECT 1 FROM foo;' | sqlformat
    sqlformat < foo.sql

FLAGS:
    -h, --help       Prints this help information
    -v, --version    Prints version information
```

Example
```bash
echo "SELECT 1 FROM foo;" | sqlformat
```

Results in:

```sql
SELECT
  1
FROM
  foo;
```
