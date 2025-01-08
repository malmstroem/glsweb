# glsweb

The GLS data portal is written in rust using the dioxus framework.
The code is a proof-of-principle project to test the feasibility of dioxus.

## deploy instructions

note: the database schema is still missing.

```bash
export SERVICE_DB_URL="postgres://<user>:<pwd>@<host>/<db>"
cd ~/glsweb
dx clean
dx build --release
pgrep glsweb | xargs kill -term
./dist/glsweb < /dev/null > run.log 2> run.err &
```
