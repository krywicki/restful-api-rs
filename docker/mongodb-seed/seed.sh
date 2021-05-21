#!/bin/sh

mongoimport --host=mongodb --username=$DB_USERNAME --password=$DB_PASSWORD --authenticationDatabase=admin \
    --db=production --collection=users --type=json --file=users.json --jsonArray
