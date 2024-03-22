# KLB Backend

# Database Migrations

Using diesel CLI

Install the CLI by following this guide: https://diesel.rs/guides/getting-started.html

Some common commands:

```
diesel migration run // execute migrations
diesel migration revert // revert the migration.
diesel migration redo // check if the migration you made is correct
```

# Settings file

Take a look at `/settings` folder. It contains some examples of config/settings files which you can fill depending on your local workstation setup.

Copy one of the templates into a `Settings.toml` file and then update the values to suit your needs.

# Docker

```
# Build
docker build -t klb-backend .

# Run container
docker run -p 127.0.0.1:3000:3000 --rm --name test-be klb-backend
```