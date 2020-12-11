# Running this example

This example depends on SQLx. You should have sqlx-cli installed.

All the code is at `src/main.rs`.

First, run the attached migrations (in `migrations/) to setup the database, and setup your environment.

```bash
>> cd examples/sqlx-postgres
>> echo DATABASE_URL=<your_postgres_database> > .env
>> sqlx database reset
```

Now run the example with
```bash
>> cargo run
```

The endpoint is setup at `/spaceship/`, e.g.

```bash
>> curl http://127.0.0.1:8082/spaceship -d '{"name": "Falcon", "num_thrusters": 16}'
{"id":1,"num_thrusters":16,"name":"Falcon"}%
>> curl http://127.0.0.1:8082/spaceship
[{"id":1,"num_thrusters":16,"name":"Falcon"}]%
>> curl http://127.0.0.1:8082/spaceship/1
{"id":1,"num_thrusters":16,"name":"Falcon"}%
>> curl -X DELETE http://127.0.0.1:8082/spaceship/1
{"id":1,"num_thrusters":16,"name":"Falcon"}%
>> curl http://127.0.0.1:8082/spaceship
[]%
```

