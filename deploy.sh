# If build fails exit now.
docker-compose up --exit-code-from build build || exit 1

# Current folder name
PROJECT_NAME=${PWD##*/}

scp target/arm-unknown-linux-gnueabihf/release/$PROJECT_NAME pi@pi:/home/pi/

# -t -t kills the process on ssh down
ssh -t -t pi@pi /home/pi/$PROJECT_NAME
