FROM mariadb:latest

# Install required tools and libraries
RUN apt-get update -yq && apt-get install -y --no-install-recommends libc6-dev gcc curl unixodbc-dev odbc-mariadb
RUN curl --proto '=https' --tlsv1.2 --silent -sSf https://sh.rustup.rs | sh -s -- --profile minimal -y

# Remove `apt-get update` metadata
RUN rm -rf /var/lib/apt/lists/*

# Install initialization scripts
COPY ./init/ /docker-entrypoint-initdb.d/

# Install ODBC configuration
COPY ./odbc/ /root/odbc_mariadb
RUN odbcinst -i -d -f /root/odbc_mariadb/mariadb_driver.ini
RUN odbcinst -i -s -f /root/odbc_mariadb/mariadb_data_source.ini
RUN rm -rf /root/odbc_mariadb

WORKDIR /root/rs-odbc
