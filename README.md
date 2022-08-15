# lnPowerSupply
This is a very overengineered desktop power supply.
Small feature list :
- Powered by Lidl or Aldi battery back
- Current limiting (not very good)
- Large display (320x240)
- Large fonts 

You can use pretty much any DC/DC converter as a core, you need to adjust R2 depending on the feedback voltage
(see lnPowerSupply.pdf)
Be careful to stay below ~ 24v else the INA219 will blow.

The overal design is made of 3 parts :
- The DC/DC converter board itself
- The DC/DC control board, controlled over i2C
- The main board with a STM32 or GD32 driving the screen

Why go to such a design ?
You need to have the control part close to the DC/DC
You need to have the SPI screen AWAY from the control board

[Small Youtube demo ](https://youtu.be/UPAjdyqt5LI)

![screenshot](assets/web1.jpg?raw=true "front")
![screenshot](assets/web2.jpg?raw=true "internal")

