# gui design

## not gui things
Starting with the non gui things, gui should be completely decoupled from all
other logic. In practice this means that all the things that *might* want their
values changed, should have a resource called `ThingSettings` that lets you
change their values form the gui, or from other means.

## gui
Gui can be split into separate things, but this does not necessarily mean that
they are in different windows, you could in theory have the spawning window
inside the performance window. I think the best way to do this is to store the
functions inside a struct for each gui.

Things like the tools gui, which are made up of the same component over and over
should actually use *the same* component instead of an identical one, like what
I am doing now. This can be done by storing functions in a struct too, I think.

Since egui wants a mutable reference to the data it is changing, I think the
best way to change the values is as follows:

1. Clone the settings resource
2. Modify values with gui
3. Overwrite the resource with the cloned one

This has the benefit of allowing much more dynamic behavior, like a mass slider
changing both density and radius.
