# Homepage

This is the repository for my [homepage](https://filipejr.com)

This project consists of 2 components:

- [Frontend](homepage-frontend/) (Rust WASM)
- [Backend](homepage-backend/) (Rust)

The frontend expects the backend to be mounted at `<frontend-location>/backend`.

The backend is a simple rest server, serving mostly json.

# Running it

To run the project, you can use [`zbuild`](github.com/Zenithsiz/zbuild) with the `run` target, which will run both the frontend and backend.

```sh
zbuild run
```

You may also run the backend or frontend separately using the `run_backend` and `run_frontend` rules, respectively.

# Deployment

To deploy the project, you can run the `deploy` rule.
This will deploy both the frontend (static html) as well as the backend (debian binary).
In the case of the backend, the systemd unit responsible for running it will be restarted.

```sh
zbuild deploy
```

You may also deploy the backend or frontend separately using the `deploy_backend` and `deploy_frontend` rules, respectively.
