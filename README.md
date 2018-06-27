# visual-rust
This is going to be a *long* project...

## Introduction
I want to build the "Visual Basic" of Rust:

* An easy to use platform where you can click and drag, resize, and reconfigure widgets on a window.
* This window should be able to compile into Rust code, cross-compatible in Windows, Mac, and Linux.
* The widgets (buttons, text boxes, etc), when compiled, should be created with the Operating System's native libraries.
* All of the widgets should have programmable event handlers, all written in Rust.

### Why..
#### Do this at all?
GUI programming is generally considered a *hard* task, especially for a beginner programmer.

Even a seasoned programmer would need to think hard about platform compatibility..

Without some kind of GUI tool, it is difficult to manage the exact positions of all of the widgets on the screen. Managing each object is a hassle in code.

The target audience is a UI designer who wants to quickly build a fast cross-compatible widget-based application.

#### Rust?
Rust is a powerful language:

1. Low Level (Fast / Optimized).
2. Easy to use, especially with Higher Order Functions.
3. All garbage collection is done at compile time.
4. Safe Concurrency and Memory Usage.

And personally, this is to expand my knowledge / abilities with Rust.

#### Native widgets?
There is a cross-compatible crate [conrod](https://crates.io/crates/conrod) that draws custom widgets.

By drawing custom widgets, exactly the applications appears exactly the same in all operating systems.

I don't like this for two reasons:

1. The look and feel of the application does not play well with the rest of operating system. It feels foreign.
2. Non-native widget drawing libraries require more memory. I want to be able to rival Notepad.exe in memory usage.

#### Visual Basic?
Visual Basic was my first language.

Although the language itself is poor, building a GUI was a piece of cake.

I want to integrate a good language - Rust - With easy GUI building from Visual Basic.

### How?
#### Phase 1

##### Part A
Write Rust code that creates an example window, with a few widgets in:

* Windows - **Done**
* Mac
* Linux

Use low level crates to build these windows and widgets.

Figure out how to hook into widget specific events handlers for each operating system.

For Linux, default to the simplest widget library for now. For example, FLTK.

##### Part B
Combine all of this code into one simple Rust windowing code.

The Rust code should have tags in it that detect which operating system you're on, and provide the correct code for that operating system.

Although we are not yet implementing event handling, it would be good to at least expose the hooks to be available for someone using this GUI tool.

Try to abstract away event handling code, to be generic enough, so that the user can use the same event handling function with every operating system.

##### Part C
Write up a HTML / Javascript "portal" that allows you to create a "window", and create "widgets" on this window.

The "windows" and "widgets" here would just be drawings of the real thing.

You should be able to switch "views" to see how the application will look like in Windows vs Mac vs Linux.

When you click on any widget, a dialog appears, that allows you to change any properties of the widget, like label, font, and color.

You should be able to click and drag,  resize, create, delete any widget.

##### Part D
Make the HTML portal have a "Build" button, that generates all of the Rust code required to compile into all 3 operating systems.

So, we need to keep all of the data, about each widget, stored in a way that can be translated into Rust code later.

##### Part E
Add in support for the rest of the Linux widget libraries (Qt and GTK).

In the HTML portal, for Linux, the user will need to select which widget library they want to build into.

The HTML portal should adjust the graphics accordingly.

The portal will need to build the Rust code for these libraries.

##### Part F
Figure out some way to Save and Open projects in the HTML portal.

Potentially this could be some kind of commented out templating in the built Rust code.

#### Phase 2
Add event handling into the HTML portal. Allow users to update the rust code of any widget's event trigger.

#### Phase ???
Make Visual-Rust "Self Hosting". Rewrite it, to compile directly from Rust.

The HTML tool should be advanced enough, were this is entirely possible to do from the portal.
