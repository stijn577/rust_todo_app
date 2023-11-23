# Yew Rocket Rust full stack

### This project is a personal project to create a fully functioning website in Rust using a [Rocket.rs](https://rocket.rs/) backend, with a [Yew.rs](https://yew.rs/) wasm frontend.

## Hosting locally
To host the website locally, you will have to perform the following actions:
```sh
cd app \ 
trunk build \ 
cd .. \ 
cargo r 
```
or when using nushell source the build.nu file using:
```sh
source build.nu
```
and then you can compile and launch the whole thing using:
```sh
launch
```

This will create the dist directory containing the index.html, wasm file and js file to link the two together. This folder is required for the server to start, otherwise it won't find the folder and refuse to start.

Once the server is up you can visit the website by going to [http://localhost:8000/](http://localhost:8000/)

*note that having trunk installed is a prerequisite, you can install it using:
```sh
cargo install trunk --locked
```

TODO: continue with the rest of the documentation