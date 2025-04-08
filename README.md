# glsweb

The GLS data portal is a data portal <https://doi.org/10.1016/j.cell.2025.03.013>.
The GLS data portal is written in rust using the [dioxus](https://dioxuslabs.com/) web framework.
It has only been tested with version 0.6.3.

## deploy instructions

The database dump can be downloaded from [zenodo](https://doi.org/10.5281/zenodo.14292542)

```bash
export SERVICE_DB_URL="postgres://<user>:<pwd>@<host>/<db>"
dx run --release
```
