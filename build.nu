def "build app" [] {
    cd app
    tailwindcss -i ./css/input.css -o ./css/output.css
    trunk build
}

def "build server" [] {
    cd server
    cargo build
}

def "build all" [] {
    build app
    build server
}

def "run app" [] {
    build app
    cd app
    trunk serve
}

    def "run server" [] {
        build server
        cd server
    cargo r
}


# def launch [] {
    # cd app
    # trunk build 
    # cd server ; cargo r 
# }

# def "clean all" [] {
#     trunk clean
#     cargo clean
# }
# 
# def "watch frontend" [] {
#     cargo watch -c -- trunk build 
# }