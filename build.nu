def launch [] {
    cd app
    trunk build
    cd ..
    cargo r
}