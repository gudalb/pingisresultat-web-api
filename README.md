# pingisresultat-web-api

## Infrastructure setup
configure .env
run: docker-compose -f docker-compose/infra-compose up -d

## Diesel setup
#### Libpg
##### 1. Windows: 
install: http://www.enterprisedb.com/downloads/postgres-postgresql-downloads
in powershell: setx PQ_LIB_DIR "C:\Program Files\PostgreSQL\{postgres_version}\lib"
reboot
##### 1. Ubuntu: 
sudo apt install libpq-dev
#### 2. Install Cli
cargo install diesel_cli --no-default-features --features postgres