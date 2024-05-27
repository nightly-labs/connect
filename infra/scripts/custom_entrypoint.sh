#!/bin/bash
echo "Starting the original entrypoint script..."

docker-entrypoint.sh "$@"
echo "Original entrypoint script has been called."

# Keep the container from exiting
tail -f /dev/null
