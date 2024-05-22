#!/bin/bash
# Execute the original entrypoint script
/docker-entrypoint.sh "$@"
# Keep the container from exiting
tail -f /dev/null
