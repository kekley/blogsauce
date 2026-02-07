+++
title = "a blog of sorts"
date = 2026-02-06
+++

[//]: # (What is this site for, a little bit about me)

## thanks for stopping by!
This is a website! (Mostly) of my own creation where I plan to share my not-so-innermost thoughts and ideas about the things I am currently working on or thinking about. 
I would like to detail what you can expect to see on this page and a little bit about its creation, but before all of that I would like to mention the joy I had helping 
a few of my friends achieve the same thing. You can find their pages at [taxidermycact.us](https://taxidermycact.us/) and [northofpolar.is](https://northofpolaris.space).  

My primary motivation for creating a website was a desire to share more about the things I find interesting or otherwise want to talk about 
while also staying away from social media and other equally shitty corporations. 
I've never been particularly fond of participating on social media or any online discourse for that matter 
so I'm still very much trying to figure out what my voice and presence on the internet actually looks and sounds like. please bear with me on that.  

Some specifics about the kind of posts you can expect to see on this page are:  
- my personal programming projects (like this site!)
- informational posts about any concept I want to familiarize myself with
- personal resolutions I want to help hold myself acountable for by sharing them publicly
- and perhaps some journal-esque content that I deem worth posting  

I will also do my best to maintain a gallery of my favorite photos, a list of media I like, and some fun interactive stuff inspired by [this page](https://snoot.org/toys/).

That wraps up the important stuff about the site if you have anything else you should really be doing right about now, 
but if you would like a little tour or some info about the development of the site then read on!

[//]: # (Features of the site)
## the 'little tour' in question
Thanks to my inability to not overbuild things, the site is currently pretty far from complete. But this is 
a rough idea of what you can expect to find on the site and where. At the top of the page you'll 
find a navigation bar with links to the four main pages of the site. 
![](/posts/post1/navbar.png)
The 'home' button will take you to the main page where you can find all of my posts sorted from 
most recent -> least. I plan on adding a comment section for each post, so the chat box will not be 
the only way to communicate on the site.

The 'gallery' button will take you to a page with a display of some of my favorite pictures. 
You can click on the images to expand them.  

The 'toys+games' button will take you to a page that is currently unfinished, but it's where 
I hope to put a bunch of little interactive demos and games that hopefully will allow some level of 
of interaction between users of the site.  

The 'about' button will take you to another work-in-progress page where I hope to write a little 
bit about myself for those that are interested in learing a bit about my interests and media I enjoy.

The site also has a live chatbox that I programmed myself! If you're on desktop it should be on the right side of the page, and if you're on mobile it'll be at the bottom of the page.
I haven't done much to make the layout nice for a vertical layout so I'm sorry if things are hard or impossible to navigate 
on a phone. 
![](/posts/post1/chat.png)
One of my first experiences interacting with people on the internet was a dinky chat room made for the web browser on the nintendo DSI. I chatted using 
my real name and one of the admins who happened to be very religious banned me. Some part of that 
experience felt fitting to try and recreate on a site like this. I'm sure there will be issues with it since it's a little hard 
to test everything that could go wrong *and* I'm not particularly familiar with 
web stuff so expect any issues you encounter to get fixed whenever I have the time. 
Anyways, you have to claim a username to use the chat, after which you'll be provided with a secret key you can think of like a password for your account, 
and a random color that your username will be displayed in. I plan on letting everyone choose their own color once I 
get around to writing the code for that. so don't worry too much if you get a color you don't like. 

[//]: # (Details about development)
## some technical stuff if you're interested
- The site was made with [Zola](https://www.getzola.org/), a static site generator meant for building... static sites.
- The backend for the chatbox and future comment system is a web server I wrote in rust using the [hyper](https://hyper.rs/) and [smol](https://docs.rs/smol/latest/smol/) crates. 
- Everything (the comments,usernames, etc..) is stored in an sqlite database because... sqlite seemed the simplest to me.




