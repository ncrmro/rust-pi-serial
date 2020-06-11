# Serial Output from Pi using Rust

A simple Blink (Hello World of microcontrollers) project that also outputs blink status on serial port.

### Enable Serial output on Pi
First open pi config.
`sudo raspi-config`
Under interfaces click serial, dont enable first option, do enable second option at next screeh.

### Finding your serial port on your local machine (not the pi)
Find your serial port on your computer (not the pi)
`ls -l /dev`

This will begin listing on the serial port.
`cat /dev/cu.usbserial-1440`

### Writing to serial port (From Pi)

Back on the pi 
`echo “Hello” > /dev/ttyS0`

Or over ssh
`ssh pi@pi "echo Hello World! > /dev/ttyS0"`

### Blink
You should now see
```
...
Blink High
Blink Low
Blink High
Blink Low
Blink High
...
```

## Building and running on Pi
If you have your ssh key setup on the pi with the default pi user just run.
This will build the binary, copy it using ssh and the executing it. The output will
automatically piped over ssh and closing the ssh will automatically kill the process on the pi.

Ensure you have docker and docker-compose available.

`sh deploy.sh`
