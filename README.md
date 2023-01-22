# Birthday Checker

Stores names with birthdays and provides a route to find the number of days
until a users next birthday.

## Usage

To submit a name and a birthday, perform a `put` request like so:

```
PUT /hello/dylan
Content-Type: application/json

{
    "dateOfBirth": "1990-01-12"
}

204 No Content
```

To check a users birthday, use a `get` request:

```
GET /hello/dylan

200 OK
content-type: application/json

{
    "message": "Hello, dylan! Your birthday is in 355 day(s)"
}

```

## Running

Assuming your environment is authenticated to access the target google cloud platform.

Using cargo:

```bash
$ env FIRESTORE_PROJECT=<my-project-id> cargo run
```

Using docker (assuming google auth envvar is set):

```bash
$ docker build . -t birthday-checker:latest
$ docker run \
  -e GOOGLE_APPLICATION_CREDENTIALS=/creds.json \
  -e FIRESTORE_PROJECT=<my-project-id> \
  -v "$GOOGLE_APPLICATION_CREDENTIALS:/creds.json" \
  birthday-checker:latest
```

## Assumptions

- Name is case sensitive, i.e. `dylan` is different from `Dylan` and `dYlAN`.
- Timezones are ignored