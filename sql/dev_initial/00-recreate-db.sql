-- DEV ONLY - Brute Force DROP DB (for local dev and unit tests)
-- Terminate all user sessions
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE 
    usename = 'app_user' OR datname = 'app_db';
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- DEV ONLY - Dev only password (for local dev and unit tests)
CREATE USER app_user WITH PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db OWNER app_user ENCODING = 'UTF-8';