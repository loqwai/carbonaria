# Idea

A game that combines Heat Signature & Castle Doctrine

2D top down game,
You control a ship that you can use to fly next to other ships.
You can go into them and take them over like in Heat Signature
You can then take parts from that ship and attach it to your own ship

You control the layout of your ship, but, the more convenient you make access, the easier it is for others to take over your ship if you are attacked.

Somehow, the things you add to the ship need to be able to help you when you're taking over enemy ships. So maybe the modules will be something like an armory for better weapons, or a robot bay, or mech suit module. Some of the stuff should be defensive, to help your ship repel boarders while you invade.

If a boarder takes out a module, you lose that ability in real time. So if they take out your robot bay, all your robots will go to sleep until you repair that module. Maybe armories would stop reloading your weapons or something?

All modules must be accessable to boarders in order for you to be able to use them. You can guard them with turrets and stuff, but it has to be possible for the enemy to walk over to them to shut them down.

## MVP

* [x] You can control a unit using WASD that can move around
* [x] The area you can move around in is bounded
* [x] There are other things in the space you can move around in
* [x] You have some kind of stick you can swing
* [x] The room is automatically generated to be something other than square.
* [x] You can hit things with the stick
* [x] The room generates tile by tile so you can watch the progress as it happens.
* [x] Some of the other things also have sticks
* [x] Those other things try and hit you with their stick when you get close
* [x] When you hit things with your stick, you get points
* [x] You can see how many points you have
* [x] When you hit things with your stick, they die.
* [x] If you get hit with a stick, it subtracts some health
* [x] If you run out of health, it's game over
* [ ] You can see how much health you have left
* [ ] There should be a place to go. Let's call it an exit
* [ ] You should always be able to reach the exit
* [ ] When you reach the exit, a new level is generated
* [ ] Those things with sticks can also move around and will chase you if you get close to them
* [ ] The camera is limited to a small circle so you can't see very far.
* [ ] The room defers generating tiles until they are about to come into view.
* [ ] Try using an ASP language like Gringo or FlatZinc to build the room generator