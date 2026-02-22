# Digital Ocean

To push to digital ocean, doctl needs to be downloaded, initialised and guthub connected.
Then one can upload the application with `doctl apps create --spec.yaml`. Both database and application is
specified in the .yaml file. Very simple but costs approx 30$/ month. So for now local hosting on rasp pi?

_Important note_
To run as prod, application needs to be ran with flag, APP_ENVIRONMENT=production
this will override `/configuration/base.yaml` with `configuration/production.yaml`

# from root dir

- create app
  doctl apps create --spec.yaml
- list
  doctl apps list
- update app specification based on spec.yaml
  doctl apps update YOUR-APP-ID --spec=spec.yaml

## getting it to work

- update app if something changed,
- push to github
- migrate database
  DATABASE_URL=YOUR_DIGITAL_OCEAN_COONNECTION_STRING sqlx migrate run

# Postman (Email client)

Currently working on school mail in sandbox mode, costs around 15$/month base level. Not worth it currently

# Local development

The docker container can be started locally, and migration run, otherwise
`cargo watch -x check -x test -x run` works perfectly, where ./scripts/init_db.sh is ran locally first.
