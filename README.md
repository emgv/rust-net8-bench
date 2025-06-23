# Rust Axum Sqlx vs .Net8 Dapper benchmark

## How to run the benchmarks
- Set the database container ip address
  - Net8: go to ```net-web-api-bench/Program.cs``` and update the placeholder ```<ip-database-container>```
  - Axum: go to ```rs-web-api/src/app.rs``` and update the placeholder ```<ip-database-container>```
- Run the app (run ```docker compose down``` first if the net8 or axum containers are running already)
  ```
  cd /path/to/net-web-api-bench or cd /path/to/rs-web-api
  docker compose up -d
  ```

## Results
- Web API request 1 random car
  - Net8: ![Net8-Cars-1](results/net8-wrk-cars-1.jpg)
  - Axum: ![Axum-Cars-1](results/rs-wrk-cars-1.jpg)
- Web API request 10 random cars
  - Net8: ![Net8-Cars-10](results/net8-wrk-cars-10.jpg)
  - Axum: ![Axum-Cars-10](results/rs-wrk-cars-10.jpg)
- Web API request 100 random cars
  - Net8: ![Net8-Cars-100](results/net8-wrk-cars-100.jpg)
  - Axum: ![Axum-Cars-100](results/rs-wrk-cars-100.jpg)
- Web API request 1000th prime
  - Net8: ![Net8-Prime-1000th](results/net8-wrk-prime-1000.jpg)
  - Axum: ![Axum-Prime-1000th](results/rs-wrk-prime-1000.jpg)
