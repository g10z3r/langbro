#!/bin/bash

echo "+++++++++++++++++++++++++++++++++"
echo $MONGO_USER
echo "+++++++++++++++++++++++++++++++++"

# Создание пользователя для БД
mongo -- "$MONGO_INITDB_DATABASE" <<EOF
    db.createUser({
        user: "$MONGO_USER",
        pwd: "$MONGO_USER_PASSWORD",
        roles: [
            {
                role: "$MONGO_USER_ROLE",
                db: "$MONGO_INITDB_DATABASE",
            },
        ],
    });
EOF

# Создание коллекций и установка индексов
mongo --username $MONGO_USER --password $MONGO_USER_PASSWORD --authenticationDatabase $MONGO_INITDB_DATABASE $MONGO_INITDB_DATABASE <<EOF
    db.createCollection("accounts");
    db.accounts.createIndex(
        {
            "username": 1
        }, 
        {
            "unique": true, 
            "partialFilterExpression": {
                "username": {
                    \$type: "string"
                }
            }
        }
    );
EOF