def launch [] {
    trunk build
    cargo r
}

def "clean all" [] {
    trunk clean
    cargo clean
}

def "watch frontend" [] {
    cargo watch -c -- trunk build 
}
