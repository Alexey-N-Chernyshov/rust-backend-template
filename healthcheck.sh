#!/bin/bash

exec 4<>/dev/tcp/127.0.0.1/8080 && echo -e "GET /health HTTP/1.0\n" >&4 && cat <&4 | grep ".*\"healthy\":true.*"
