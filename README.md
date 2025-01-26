# Homepage

This is the repository for my [homepage](https://filipejr.com)

This project consists of 2 components:

- [Frontend](frontend/) (Rust WASM)
- [Backend](backend/) (Rust)

The frontend expects the backend to be mounted at `<frontend-location>/backend`.

The backend is a simple rest server, serving mostly json.

# Running it

To run the project, you can run the [scripts/run.sh](scripts/run.sh) script.
This will start the backend and frontend in parallel.

You may also run the [backend](scripts/run-backend.sh) or [frontend](scripts/run-frontend.sh) scripts separately:

```sh
# Note: Don't `cd` into scripts to run them, run them from the project root.
scripts/run.sh
```

# Deployment

To deploy the project, you can run the [scripts/deploy.sh](scripts/deploy.sh) script.
This will deploy both the frontend (static html) as well as the backend (debian binary).
In the case of the backend, the systemd unit responsible for running it will be restarted.

You may also deploy the [backend](scripts/deploy-backend.sh) or [frontend](scripts/deploy-frontend.sh) scripts separately:

```sh
scripts/deploy.sh
```
