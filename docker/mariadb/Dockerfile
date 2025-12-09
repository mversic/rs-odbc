FROM mariadb:latest

RUN set -eux; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        curl \
        gcc \
        libc6-dev \
        odbcinst \
        unixodbc-dev; \
    \
    # NOTE: MariaDB ODBC has a bug in 3.1.15
    arch="$(dpkg --print-architecture)"; \
    echo "Detected architecture: $arch"; \
    \
    case "$arch" in \
        amd64)  odbc_url="https://dlm.mariadb.com/4465922/Connectors/odbc/connector-odbc-3.2.7/mariadb-connector-odbc_3.2.7-1+maria~noble_amd64.deb" ;; \
        arm64)  odbc_url="https://dlm.mariadb.com/4465912/Connectors/odbc/connector-odbc-3.2.7/mariadb-connector-odbc_3.2.7-1+maria~noble_arm64.deb" ;; \
        *)      echo "Unsupported architecture: $arch"; exit 1 ;; \
    esac; \
    \
    # Download and install the correct driver
    curl -Lf -o mariadb-odbc.deb "$odbc_url"; \
    apt-get install -y ./mariadb-odbc.deb; \
    rm mariadb-odbc.deb; \
    \
    # Install Rust toolchain
    curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | sh -s -- --profile minimal -y; \
    \
    # Cleanup
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*

COPY ./init/ /docker-entrypoint-initdb.d/
COPY ./odbc/ /root/odbc_mariadb

RUN set -eux; \
    arch="$(dpkg --print-architecture)"; \
    case "$arch" in \
        amd64) driver_path="/usr/lib/x86_64-linux-gnu/libmaodbc.so" ;; \
        arm64) driver_path="/usr/lib/aarch64-linux-gnu/libmaodbc.so" ;; \
        *) echo "Unsupported arch: $arch"; exit 1 ;; \
    esac; \
    sed -i "s|__ODBC_DRIVER_PATH__|$driver_path|g" /root/odbc_mariadb/mariadb_driver.ini; \
    odbcinst -i -d -f /root/odbc_mariadb/mariadb_driver.ini; \
    odbcinst -i -s -f /root/odbc_mariadb/mariadb_data_source.ini; \
    rm -rf /root/odbc_mariadb

WORKDIR /root/rs-odbc
