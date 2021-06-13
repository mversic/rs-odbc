FROM mariadb:latest

# Install required tools and libraries
RUN apt-get update && apt-get install -y unixodbc-dev odbc-mariadb curl
RUN curl --proto '=https' --tlsv1.2 --silent -sSf https://sh.rustup.rs \
    | sh -s -- --profile minimal -y

# Install initialization scripts
COPY ./init/ /docker-entrypoint-initdb.d/

# Install ODBC configuration
COPY ./odbc/ /root/odbc_mariadb
RUN odbcinst -i -d -f /root/odbc_mariadb/mariadb_driver.ini
RUN odbcinst -i -s -f /root/odbc_mariadb/mariadb_data_source.ini
RUN rm -rf /root/odbc_mariadb

WORKDIR /root/rs-odbc
