## stm32f3-Accelerometer
 **This template will provide you the way to work with the stm32-discovery board's sensor as an **Accelerometer**. 
 This template is going to use the sensor of the stm32f3 board by extracting the readings of the Accelerometer Sensor.**
 
**It is going to generate the Acceleration from the Accelerometer Sensor in all three x, y, z axis.**

## What I am using here to complete the challenge.

- **Rust Programming Language**
- **A stm32f3 Discovery Board**
- **Few dependencies**

## What we are actually doing here?

Here in this mini project we are going to get the readings from the Accelerometer Sensor and will measure the acceleration.

We are going to get the readings of the Sensor by keeping it in idle state(without moving it) and therefore we will get 
the real readings in z-azis due to gravitational pull in downward direction.

We are going to get the positive readings in z-axis due to [Proper-Acceleration](https://en.wikipedia.org/wiki/Proper_acceleration).

All these readings are going to print on the itm-dump console.

## Build the Project

**Now before building this project you need to solder your board. It will help in printing the data to itm terminal. 
Use this [link](https://docs.rust-embedded.org/discovery/06-hello-world/index.html) to solder your f3 Board.

### Note: Make sure the F3 Board is connected to your computer.

### Step 1:
Open terminal from home directory and execute Command

`cd /tmp && touch itm.txt`

Then

`itmdump -F -f itm.txt`

Leave this terminal running. Now in new terminal run command.

`cd /tmp && openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`

### Step 2:
Execute the Command

`cargo run`

Then we will be in gdb terminal. Now execute command:

`step`

`continue`