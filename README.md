# Api manager service backend

Serving API Gateway manager administrative pages

## Building process

The package is built by multi-stage Dockerfile (https://docs.docker.com/build/building/multi-stage/)

Stages and produced code are:

1. build react application: it produce index.html, index.bundle.js, index.bundle.js.map
2. build the rust executable: it produce apimanager-service executable
3. deploy executable in a scratch debian image with assets

The rust build process expects ENV variable specifying the path of static files to be loaded and served in the path, i.e.:

> ASSETS=/assets cargo build --release


## TODO

- serving static pages - OK
- serving api staff - OK
- attach utility service: service-manager-service - OK
- accept ENV execution variable - OK