def launch [] {
    trunk build
    cargo r
}

def "clean all" [] {
    trunk clean
    cargo clean
}