---
title: "Texor"
author: "Erox-02"
description: "My ultra cool keyboard"
created_at: ""
---

# Oct 6 

today imma gonna start the keyboard .

At first i need switches , so imma gonna find them .

After a lot of searching(likely 30+ min on alibaba and amazon ) i found a good one 

![](assets/sw-fnd.png)

but unfortunately they didnt tag any datasheet so imma gonna use the standard outemu stich model as the dimensions match 

eos for today 

**Total time spent: 1 hours**

# Oct 7 

so today i started with the kicad side , added basic symbols and footprints for the switches only but still it isnt same as the switch in my cart , idk wht happened.

pk placed all the switchs on sch , it is currently 12x7 i placed it , idk why but it turned out

ok so now imma gonna do a really clever thing for the gpio's but its gonna need more diode nah the combination one gonna take way more diodes , i dunno wanna do tht so basic 12x7 matrix ah nah thts too basic , but lets do tht anyways .

added `1N4148W` diode for each key now connection ,

![switchs on kb](assets/ksw.png)

okk connected every thing up now i thing gonna add the mcu 

![connected](assets/conn.png)

added the esp and read the datasheet a bit heres wht i found

![](assets/strapp.png)

the strapping pins 

so i needa pull gpio 0 low to get into dowload otherwise high and gpio45,46 floating , easy peasy 

ok added the usb c , now gotta figure out how to connect this one 

![](assets/usbc.png)

done connected the 

![](assets/usb-dn.png)

now i need a ic to controll the led's so i thing imma gonna use `74AHCT1G125` , this one is really cheap on robu also it is kinda popular for one gpio multi led driver , i think its a psuedo i2c or semi i2c after reading the datasheet anyways gonna use this.

![](assets/en-ic.png)
---

as i said its really cheap , now gonna wire it .

i am pretty lucky , kicad had this on sym i didnt had to fetch it with easyeda2kicad lol

![](assets/en-ic-sch.png)

wired it , also lin means led in .

i tried to find the perfect led for my project a lot but it was either too expensive or needs a driver and after all i concluded adding a driver will be cheaper hmpf .

js check it out , its sooooo cheap 

![](assets/ledd.png)

where the addressable one's cost 10-14 rs each hell naw imma not gonna use those even it makes things bulky , as who wants to waste money on ic embedded led , i wd rather put a bigger ic and use i2c. 

OK I GOT A HOLY IDEA , i can use multiple `STM32F030C6T6` and then make the keyboard modular as hell i mean the stm's will do the whole led + switches works while esp
 will have each stm with i2c + the display i can literally do custom i2c for it , it will be considered the worlds most modular keyboard lol . anyways for this i will need to redesign the switch matrix , ah thts a lot of work again .

 wait isnt using stm32's for each part a bit way too much inefficient? 

 nah lets go back to the costlier `WS2812B-2020` or the cost will be equal anyways .

 ok done i was lucky tht kicad already had the sym and did the wiring , bruh it was tiring 

![](assets/leddd.png)

k enough gonna do the esp + switch matrix connection tomorrow .

**Total time spent: 3 hours**

# Oct 8

today i though i wd place the keys on right position so i went ahead and used open source tools to make the json ladout and then when i used kbplacer on my pcb ,
boom:
![](assets/errr.png)

and then i used relative diode , then sw1 not found lol , then i fixed and upgraded the pcb , boom:
```
Traceback (most recent call last):
  File "/home/erox/.local/share/kicad/10.0/3rdparty//plugins/com_github_adamws_kicad-kbplacer/kbplacer_plugin_action.py", line 100, in Run
    self.__run()
    ~~~~~~~~~~^^
  File "/home/erox/.local/share/kicad/10.0/3rdparty//plugins/com_github_adamws_kicad-kbplacer/kbplacer_plugin_action.py", line 84, in __run
    run_from_gui(self.pcb_file_path, gui_state)
    ~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/erox/.local/share/kicad/10.0/3rdparty//plugins/com_github_adamws_kicad-kbplacer/kbplacer_plugin.py", line 175, in run_from_gui
    return run_board(settings)
  File "/home/erox/.local/share/kicad/10.0/3rdparty//plugins/com_github_adamws_kicad-kbplacer/kbplacer_plugin.py", line 119, in run_board
    placer.run(
    ~~~~~~~~~~^
        settings.layout_path,
        ^^^^^^^^^^^^^^^^^^^^^
    ...<8 lines>...
        encoder_adjustment=settings.encoder_adjustment,
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    )
    ^
  File "/home/erox/.local/share/kicad/10.0/3rdparty//plugins/com_github_adamws_kicad-kbplacer/key_placer.py", line 1255, in run
    self.remove_dangling_tracks()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/home/erox/.local/share/kicad/10.0/3rdparty//plugins/com_github_adamws_kicad-kbplacer/key_placer.py", line 559, in remove_dangling_tracks
    for track in self.board.GetTracks():
                 ~~~~~~~~~~~~~~~~~~~~^^
  File "/usr/lib/python3.14/site-packages/pcbnew.py", line 22181, in GetTracks
    def GetTracks(self):              return list(self.Tracks())
                                             ~~~~^^^^^^^^^^^^^^^
  File "/usr/lib/python3.14/site-packages/pcbnew.py", line 12810, in __iter__
    item = it.next()  # throws StopIteration when iterator reached the end.
           ^^^^^^^
AttributeError: 'SwigPyIterator' object has no attribute 'next'
```

its a internal err even worse , heh fixed tht err , then the keyboard wasnt accepting the diodes but did the leds so  i later interchanged the names and froze the leds lol and now its done 

![](assets/kdone.png)

wait i named the file kdone , did the kde spirit possesed me for a sec? i dont use kde anyways .

ok done i plugged the matrix pins correctly also added test point to boot into the bootloader i mean i just connected gpio0 to a test point also avoided all the strapping pins and psram pins , wait i forgot gpio3

![](assets/esp_sch.png)

lemme fix the gpio3

done 

![](assets/esp-sch.png)

bruh still placed a pin away ok fixed.

ohh wait i forgot eeprom , i avoided psram but eepromm ahhhhhhh , wait my pins arent connected to those pins at all hell yeah  i knew it .

oh r7 was conflicting , so i just put tht frm io26 to io48 done no more prob . 

ok then i added a 220 ohm res before the 1st led and the logic ic , now all is left to connect the switches and a lot of oh i barely missed tht .

ok connected all the 5v of the led , also connected gnd to all the leds in matrix uf 

![](assets/gnd_conn.png)

oh connected the rows and next led data line :

![](assets/uf.png)

done for now

**Total time spent: 3 hours**