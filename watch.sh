cargo watch -C 'src' -i '/www' -x 'build --target wasm32-unknown-unknown --release' -s '(mv --force ../target/wasm32-unknown-unknown/release/responsive_game.wasm ../www/game.wasm && notify-send "✔    Cargo Watch" "All building process have been done successfully") || notify-send "❌    Cargo Watch" "Build not successfully runs; please check the compile error messages"' & 
basic-http-server www
