#!/bin/bash


# Load database connection info

# Read query into a variable
sql="$(<"users_info.sql")"


psql --version
# Connect to the database, run the query, then disconnect
PGPASSWORD=postgres psql --host postgres.default.svc.cluster.local -U postgres -d postgresdb -p 5432 -t -A -c "${sql}"
#psql -t -A -c "${sql}"



