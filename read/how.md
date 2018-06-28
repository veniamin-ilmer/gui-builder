# How?
## Phase 1

### Part A
Write Rust code that creates an example window, with a few widgets in:

* [x] Windows
* [ ] Mac
* [x] Linux

Use low level crates to build these windows and widgets.

Figure out how to hook into widget specific events handlers for each operating system.

For Linux, default to GTK.

### Part B
Combine all of this code into one simple Rust windowing code.

The Rust code should have tags in it that detect which operating system you're on, and provide the correct code for that operating system.

Although we are not yet implementing event handling, it would be good to at least expose the hooks to be available for someone using this GUI tool.

Try to abstract away event handling code, to be generic enough, so that the user can use the same event handling function with every operating system.

### Part C
Write up a HTML / Javascript "portal" that allows you to create a "window", and create "widgets" on this window.

The "windows" and "widgets" here would just be drawings of the real thing.

You should be able to switch "views" to see how the application will look like in Windows vs Mac vs Linux.

When you click on any widget, a dialog appears, that allows you to change any properties of the widget, like label, font, and color.

You should be able to click and drag,  resize, create, delete any widget.

### Part D
Make the HTML portal have a "Build" button, that generates all of the Rust code required to compile into all 3 operating systems.

So, we need to keep all of the data, about each widget, stored in a way that can be translated into Rust code later.

### Part E
Figure out some way to Save and Open projects in the HTML portal.

Potentially this could be some kind of commented out templating in the built Rust code.

### Part F
Add in support for the rest of the Linux widget libraries (Eg: Qt).

In the HTML portal, for Linux, the user will need to select which widget library they want to build into.

The HTML portal should adjust the graphics accordingly.

The portal will need to build the Rust code for these libraries.

## Phase 2
Add event handling into the HTML portal. Allow users to update the rust code of any widget's event trigger.

## Phase ???
Make Visual-Rust "Self Hosting". Rewrite it, to compile directly from Rust.

The HTML tool should be advanced enough, were this is entirely possible to do from the portal.
