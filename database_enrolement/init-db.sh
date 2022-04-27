#!/bin/bash

psql -U postgres -d database_name -c "SELECT c_defaults  FROM user_info WHERE c_uid = 'testuser'"