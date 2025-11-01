

# Docker instance
```bash
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:17
```
## Delete process running in port 5432

```bash
sudo lsof -n -i :5432 | grep LISTEN 
sudo kill ...
```

# psql terminal on pg. 
In another terminal (tab) run psql:
```bash
docker exec -it -u postgres pg psql
```
In pgsql console
```bash
# print all sql statements.
ALTER DATABASE postgres SET log_statement = 'all';

# Connect to db
\c app_db

# List all tables
\d

select * from "user";

# describe specific table
\d task
```
