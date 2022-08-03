cd frontend
trunk build --release 
cd ..
ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=$PORT ROCKET_KEEP_ALIVE=0 ./target/release/backend