# RS-ODBC

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/mversic/rs-odbc/blob/master/LICENSE)
![Build](https://github.com/mversic/rs-odbc/actions/workflows/odbc-ci.yml/badge.svg)

ODBC implementation that looks and feels like ODBC

# Testing

Integration tests use dockerized environment which has database and ODBC driver already set up.

Testing environment can be set up with `docker-compose up -d`<br/>
Tests can be executed with `docker exec -t rs-odbc cargo test`
