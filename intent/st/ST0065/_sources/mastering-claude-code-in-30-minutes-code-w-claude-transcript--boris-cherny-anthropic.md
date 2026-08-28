# Mastering Claude Code in 30 Minutes | Code w/ Claude [Boris Cherny]

https://www.youtube.com/watch?v=B_KAEqiC-0Q

## Transcript

Search transcript

0:12

12 seconds

[music]

0:13

13 seconds

[applause]

0:16

16 seconds

Hello.

0:18

18 seconds

Hey everyone. Uh, I'm Boris. I'm a member of technical staff here at Anthropic and I created quad code and

0:26

26 seconds

here to talk to you a little bit about some practical tips and tricks for using quad code. Um it's going to be very practical. I'm not going to go too much

0:33

33 seconds

into the history or the theory or anything like this. Uh and yeah, before we start actually can we get a quick show of hands? Who has used quad code before?

0:44

44 seconds

Yeah. All right. That's what we like to see. For everyone that didn't raise your hand, uh I know you're not supposed to do this while people are talking, but if

0:51

51 seconds

you can open your laptop and type this and this will help you install quad code

0:59

59 seconds

uh just so you can follow along for the rest of the talk.

1:07

1 minute, 7 seconds

All you need is NodeJS. If if you have it, this should work.

1:16

1 minute, 16 seconds

Yeah, if you well, you don't have to you don't have to follow along, but if you don't have it yet, yeah, this is your chance to install it, so you can follow along.

1:25

1 minute, 25 seconds

So, what is quad code? Cloud code is a new kind of AI assistant. And there's been different generations of AI

1:33

1 minute, 33 seconds

assistance for coding. Most of them have been about completing, you know, like a line at a time, completing a few lines of code at a time. Cloud code is not for

1:41

1 minute, 41 seconds

that. It's fully agentic. So, it's meant for building features, for writing entire functions, entire files, fixing entire bugs at at the same time.

1:52

1 minute, 52 seconds

And what's kind of cool about cloud code is it works with all of your tools and you don't have to change out your workflow, you don't have to swap everything to start using it. So,

2:00

2 minutes

whatever IDE you use, if you use VS Code or if you use Xcode or if you use uh Jet Brains IDEs, there's some people at

2:08

2 minutes, 8 seconds

Anthropic that you can't pry them from their cold dead hands, but they use Cloud Code because Cloud Code works with every single ID, every terminal out

2:16

2 minutes, 16 seconds

there. It'll work locally uh over remote SSH, over T-Mo, whatever environment you're in, you can run it.

2:26

2 minutes, 26 seconds

It's general purpose and this is something where if you haven't used these kind of free form coding assistants in the in the past, it can be

2:34

2 minutes, 34 seconds

kind of hard to figure out how to get started because you open it up and you just see a prompt bar and you might wonder like what do I what do I do with this? What do I type in? It's a power

2:42

2 minutes, 42 seconds

tool so you can use it for a lot of things. Um, but also because it can do so much, we don't try to guide you towards a particular workflow because really you should be able to use it however you want as an engineer.

2:57

2 minutes, 57 seconds

As you open up Cloud Code for the first time, there's a few things that we recommend doing uh to get your environment set up. And these are pretty straightforward. So run terminal setup.

3:05

3 minutes, 5 seconds

This will give you shift enter for new lines. So you don't have to do like backslashes to enter new lines. This is, you know, it it makes it a little bit

3:11

3 minutes, 11 seconds

nicer to use. Do theme to set light mode or dark mode or doltonize themes. Um you can do slashinstall github app.

3:20

3 minutes, 20 seconds

So today when we announced a GitHub app where you can at mention uh claude on any GitHub issue or pull request. So to

3:29

3 minutes, 29 seconds

install it just run this command in in your terminal.

3:33

3 minutes, 33 seconds

Um you can customize the set of allowed tools that you can use so you're not prompted for it every time. This is pretty convenient. Um for stuff that I'm prompted about a bunch I'll definitely

3:40

3 minutes, 40 seconds

customize it in this way so I don't have to accept it every time. And something that I actually do is for a lot of my prompts I won't hand type them into

3:48

3 minutes, 48 seconds

cloud code. If you're on Mac OS, you can go into your system settings under accessibility is dictation and you can enable it. And so something I do is you

3:56

3 minutes, 56 seconds

just hit like that dictation key twice and you can just speak your prompt.

4:01

4 minutes, 1 second

And it helps a lot to have specific prompts. So this is actually pretty awesome. You can just talk to cloud code and uh like you would another engineer and you don't have to type a lot of code.

4:14

4 minutes, 14 seconds

So when you're starting out with cloud code, it's so free form and it can do everything. What do you start with? The thing I recommend above everything else

4:21

4 minutes, 21 seconds

is starting with codebased Q&A. So just asking your question, asking questions to your codebase. This is something that we teach new hires at Anthropic. So on

4:30

4 minutes, 30 seconds

the first day in technical onboarding, you learn about cloud code. You download it, you get it set up, and then you immediately start asking questions about the codebase. And in the past when

4:39

4 minutes, 39 seconds

you're doing technical onboarding, it's something that taxes the team a lot, right? You have to ask other engineers on the team questions. You have to look around the code and this takes a while.

4:47

4 minutes, 47 seconds

You have to figure out how to use the tools. That t this takes a long time.

4:51

4 minutes, 51 seconds

With quad code, you can just ask quad code and it'll explore the codebase. It'll answer these kind of questions.

4:56

4 minutes, 56 seconds

And so at Enthropic onboarding used to take about two or three weeks for technical hires. It's now about two or three days.

5:05

5 minutes, 5 seconds

What's also kind of cool about Q&A is we uh we don't do any sort of indexing. So there's no remote database with your code. We don't upload it anywhere. Your

5:12

5 minutes, 12 seconds

code stays local. We do not train generative models on the code. So it's there, you control it. There's no indices or anything like this. And what

5:21

5 minutes, 21 seconds

that means is also there's no setup. So you start cla you download it, you start it. There's no indexing. You don't have to wait. You can just use it right away.

5:30

5 minutes, 30 seconds

This is a technical talk. So um I'm going to show some very specific prompts and very specific code samples that you can use and hopefully improve and up

5:37

5 minutes, 37 seconds

your quad code experience. So some kind of questions that you can ask is uh you know like how is this particular piece of code used or how do I instantiate

5:46

5 minutes, 46 seconds

this thing and cloud code it won't just do like a text search and try to answer this. It'll often go a level deeper and it'll try to find examples of how is

5:53

5 minutes, 53 seconds

this class instantiated how is it used and it'll give you a much deeper answer.

5:57

5 minutes, 57 seconds

So something that you would get out of a wiki or documentation instead of just like command f something that I do a lot also is ask it

6:06

6 minutes, 6 seconds

about git history. So for example, you know, why does this function have 15 arguments? And why are the arguments named this weird way? And this is

6:13

6 minutes, 13 seconds

something I bet in all of our code bases, you have some function like this or some class like this.

6:19

6 minutes, 19 seconds

And cloud code can look through git history and it'll look to figure out how did these arguments get introduced and who introduced them and what was the situation? What are the issues that

6:26

6 minutes, 26 seconds

those commits linked to? And it'll look through all this and summarize it. And you don't have to tell it that in all these in all this detail. You just ask it. So just say look through git history and it'll know to do this.

6:38

6 minutes, 38 seconds

The reason it knows by the way is not because we prompted it to. There's nothing in the system prompt about looking through git history. It knows it because the model is awesome and if you

6:46

6 minutes, 46 seconds

tell it to use git it'll know how to use git. So we're lucky to be building on such a good model.

6:52

6 minutes, 52 seconds

I often ask about uh GitHub issues. So um you know it can use web fetch and it can fetch issues and woke up context on issues too. And this is pretty awesome.

7:02

7 minutes, 2 seconds

Uh, and this is something that I do every single Monday in our weekly standup is I ask, "What did I ship this week?" And Quad Code looks the log, it

7:10

7 minutes, 10 seconds

knows my username, and it'll just give me a nice readout of everything I I shipped. And I'll just copy and paste that into a document.

7:20

7 minutes, 20 seconds

So, yeah, that's tip number one. For people that have not used Cloud Code before, if you're just showing it to someone for the first time, on boarding your team, the thing we definitely recommend is start with codebased Q&A.

7:30

7 minutes, 30 seconds

Don't start by using fancy tools. Don't start by editing code. Just start by asking questions about the codebase. And that'll teach people how to prompt. And it'll start teaching them this boundary

7:38

7 minutes, 38 seconds

of like what can claude code do? What is it capable of versus what do you need to hold its hand with a little bit more?

7:44

7 minutes, 44 seconds

What can be oneshotted? What can be twoshotted, threeshotted? What do you need to use interactive mode for in a ripple?

7:52

7 minutes, 52 seconds

Once you're pretty comfortable with Q&A, you can dive into editing code. This is the next thing. And the cool thing about

7:59

7 minutes, 59 seconds

uh any sort of agentic uh you know like using a LM in an agentic way is you give it tools and it it's just like magical.

8:06

8 minutes, 6 seconds

It figures out how to use the tools. And with cloud code we give it a pretty small set of tools. It's not a lot. And so it has a tool to edit files. It has a

8:14

8 minutes, 14 seconds

tool to run bash commands. Uh it has a tool to search files. And it'll string these together to explore the code, brainstorm and then finally make edits.

8:24

8 minutes, 24 seconds

And you don't have to prompt it specifically to use this tool and this tool and this tool. just say, you know, do this thing and it'll figure out how to do it. It'll string it together in the right way that makes sense for quad code.

8:37

8 minutes, 37 seconds

There's a lot of ways to use this.

8:38

8 minutes, 38 seconds

Something I like to do sometimes is before having Claude jump in to write code, I'll ask it to brainstorm a little

8:45

8 minutes, 45 seconds

bit or make a plan. This is something we highly recommend and something I see sometimes is people, you know, they take quad code and they ask it, hey,

8:53

8 minutes, 53 seconds

implement this enormous like 30,000 line uh feature and sometimes it gets this right on the first shot. But sometimes what happens is the thing that it builds is not at all the thing that you wanted.

9:03

9 minutes, 3 seconds

And the easiest way to get the result you want is ask it to think first. So brainstorm ideas, make a plan, run it by me, ask for approval before you write

9:12

9 minutes, 12 seconds

code. And you don't have to use plan mode. You don't have to use any special tools to do this. All you have to do is ask claud and it'll know to do this. So

9:19

9 minutes, 19 seconds

just say before you write code, make a plan. That's it.

9:25

9 minutes, 25 seconds

This is also I wanted to include this one. This commit push. This is a really common indentation that I use. There's nothing special about it, but Claude is kind of smart enough to interpret this.

9:32

9 minutes, 32 seconds

So it'll make a commit. It'll push it to the branch, make a branch, and then make a pull request for me on GitHub. You don't have to explain anything. It'll look through the code. It'll look through the history. It'll look through

9:41

9 minutes, 41 seconds

the git log by itself to figure out the commit format and all the stuff. and it'll make the commit and push it the right way.

9:48

9 minutes, 48 seconds

Again, we're not system prompting it to do this. It just knows how to do this. The model is good.

9:56

9 minutes, 56 seconds

As you get a little bit more advanced, you're going to want to start to plug in your team's tools. And this is where Cloud Code starts to really shine. And there's generally two kinds of tools.

10:04

10 minutes, 4 seconds

So, one is batch tools. And an example of this, I just made up this like barley CLI. This isn't a real thing. Um, but you can say use this CLI to do

10:12

10 minutes, 12 seconds

something. And you can tell cloud code about this and you can tell it to use for example like d-help to figure out how to use it. And this is efficient. If

10:21

10 minutes, 21 seconds

you find yourself using it a lot, you can also dump this into your cloud MD which we'll talk about in a bit. So cloud can remember this across sessions.

10:27

10 minutes, 27 seconds

But this is a common pattern we follow at anthropic and we see external customers use too. And same thing with MCP. Um quad code can use batch tools,

10:36

10 minutes, 36 seconds

it can use MCP tools. So, you know, just tell it about the tools and you can add the MCP tool and you can tell it how to use it and it'll it'll just start using it.

10:45

10 minutes, 45 seconds

And this is extremely powerful because when you start to use code on a new codebase, you can just give it all of your tools, all the tools your team

10:52

10 minutes, 52 seconds

already uses for this codebase and cloud code can use it on your behalf.

11:02

11 minutes, 2 seconds

There's a few common workflows and this is the one that I talked about already. So kind of do a little bit of exploration, do a little bit of planning

11:10

11 minutes, 10 seconds

and ask ask me for confirmation before you start to write code. These other two on the right are extremely powerful when

11:17

11 minutes, 17 seconds

cla has some way to check its work. So for example by writing unit tests or screenshotting in puppeteer or screenshotting the iOS simulator then it

11:26

11 minutes, 26 seconds

can iterate and this is incredible because if you give it for example a mock and you say build this web UI it'll get it pretty good. But if you let it

11:34

11 minutes, 34 seconds

iterate two or three times, often it gets it almost perfect. So the trick is give it some sort of tool that it can use for feedback to check its work and

11:43

11 minutes, 43 seconds

then based on that it will iterate by itself and you're going to get a much better result. So whatever your domain is, if it's unit test or integration

11:49

11 minutes, 49 seconds

test or screenshots for apps or web or anything, just give it away to see its result and it'll iterate and get better.

12:00

12 minutes

So these are the next steps. teach Quad how to use your tools and figure out the right workflow. Um, if you want Quad to jump in a code, if you want it to brainstorm a little bit, make a plan, if

12:09

12 minutes, 9 seconds

you want it to iterate, kind of have some sense of that so you know how to prompt Quad to do what you want.

12:17

12 minutes, 17 seconds

As you go deeper beyond tools, you want to start to give Quad more context. And the more context, the smarter the decisions will be because as an engineer

12:25

12 minutes, 25 seconds

working in a codebase, you have a ton of context in your head about your systems and all the history and and everything else. So you can there's different ways to give this to quad and as you give quad more context it'll do better.

12:37

12 minutes, 37 seconds

There's different ways to do this. The simplest one is what we call quad MD and quad.md is the special file name. The

12:45

12 minutes, 45 seconds

simplest place to put it is in the project route. So the same directory you start quad in. Put a quad MD in there and that'll get automatically read into

12:53

12 minutes, 53 seconds

context at the start of every session and essentially the first user turn will include the quad MD.

13:00

13 minutes

You can also have a local cloudmomd and uh this one you don't usually check into source control. So cloudmd you should check into source control share with

13:08

13 minutes, 8 seconds

your team so that you can write it once and share it with your team. This one you don't check in it's just for you.

13:15

13 minutes, 15 seconds

The kinds of things you put in quadd it's like common bash commands common MCP tools uh architectural decisions important files anything that you would

13:24

13 minutes, 24 seconds

kind of typically need to know in order to work in this codebase. Try to keep it pretty short because if it gets too long, it's just going to use up a bunch of context and it's usually not that

13:32

13 minutes, 32 seconds

useful. So, just try to keep it as short as you can. And for for example, in our codebase, we have uh common batch commands, we have a style guide, we have

13:41

13 minutes, 41 seconds

a few core files, kind of things like that. All the other quad MDs, you can put them in other nested child directories and claude will pull them in on demand.

13:50

13 minutes, 50 seconds

So, these are the quadds that will get pulled in automatically. Um but then also you can put in put quadmds in nested directories and those will get

13:58

13 minutes, 58 seconds

put those will get automatically pulled when cloud works in those directories.

14:03

14 minutes, 3 seconds

Um and of course if you're you know a company maybe you want a quadmd that's shared across all the different code bases and you want to manage it on behalf of your users and you can put it

14:10

14 minutes, 10 seconds

in your enterprise route and that'll get pulled in automatically.

14:17

14 minutes, 17 seconds

There's a ton of ways to pull in context. I actually had a lot of trouble putting this slide together just to communicate the breadth of ways you can do this. But quadmd is pulled in

14:25

14 minutes, 25 seconds

automatically. You can also use slash commands. So this is dotquad/comands and this can be in your home directory or it can be checked into your project. And

14:33

14 minutes, 33 seconds

this is for slash commands. And over here we have a few examples of the slash commands that we have in cloud code

14:42

14 minutes, 42 seconds

itself. And so for example, if you're in the cloud code repo and you see issues getting labeled, that's actually this workflow running here. It's label GitHub

14:50

14 minutes, 50 seconds

issues and we have a GitHub action running, the same one we talked about this morning where cloud code will run this command

14:58

14 minutes, 58 seconds

and it's just a slash command. It'll run and it'll label the issues so humans don't have to. It just saves us a bunch of time.

15:04

15 minutes, 4 seconds

And of course, you can atmention files to pull them into context. Um, and like I said before, QuadMDs in a nested directory get pulled in when Quad works in that directory.

15:18

15 minutes, 18 seconds

So, give Quad more context. And it's definitely worth taking the time to tune context. You can run it through a prompt improver. Consider who the context is

15:26

15 minutes, 26 seconds

for. If you want to pull it in every time, if you want to pull it in on demand, if you want to share it with a team, if it's a personal preference, definitely take the time to tune it.

15:34

15 minutes, 34 seconds

This will improve performance dramatically uh if you do it right.

15:42

15 minutes, 42 seconds

As you get more advanced, you're going to want to think about this a little bit more. This kind of hierarchy of different ways to pull in everything. So

15:48

15 minutes, 48 seconds

like not just cloudmd, but also config and uh kind of everything about quad you can pull in in this hierarchical way. So

15:57

15 minutes, 57 seconds

projects uh are specific to your git repo and this you can check in or you can make it just for you. You can also have global configs that are across all

16:05

16 minutes, 5 seconds

your projects or you can have enterprise policies and this is essentially a global config that you row out for all of your employees, everyone on your team automatically.

16:14

16 minutes, 14 seconds

And this slide is like pretty information dense, but the point is this applies to a lot of stuff. So you can do this for slash commands, you can do it for permissions. So for example, if you

16:22

16 minutes, 22 seconds

have a batch command that you would run for all your employees uh like all your employees use this like test command for example, you can actually just check it

16:30

16 minutes, 30 seconds

into this enterprise policies file and then any employee when they run this command, it will be auto approved which is pretty convenient. And you can also

16:38

16 minutes, 38 seconds

use this to block commands. So for example, let's say there's a URL that should never be fetched. Um just add it to this config and that'll make it so an

16:45

16 minutes, 45 seconds

employee cannot overwrite it and that that URL can never be fetched. So pretty convenient both to unblock people and also just to keep your codebase safe.

16:55

16 minutes, 55 seconds

And then same thing for MCP servers.

16:56

16 minutes, 56 seconds

Have an MCP JSON file, check it into the codebase. That way anytime someone runs quad code in your codebase, they'll be prompted to install the MCP servers and share it with the team.

17:10

17 minutes, 10 seconds

If you're not sure which of these to use, this is like a kind of an insane matrix because we support a lot of stuff and engineer workflows are very flexible and every company is different. So we

17:17

17 minutes, 17 seconds

kind of want to support everything. So if you're not sure how to get started, I would recommend start with shared project context.

17:24

17 minutes, 24 seconds

You write this once and then you share it with everyone on the team and you get this kind of network effect where you know someone does a little bit of work and everyone on the team benefits.

17:35

17 minutes, 35 seconds

There's a lot of tools built into cloud to manage this. Uh so as an example, if you run /memory, you can see all the different memory files that are getting

17:42

17 minutes, 42 seconds

pulled in. So maybe I have an enterprise policy. I have my user memory. I have project quad MD and then maybe there's a nested quad MD that's only pulled in for certain directories.

17:54

17 minutes, 54 seconds

And then similarly when you do SL memory you can edit particular memory files.

17:57

17 minutes, 57 seconds

When you type pound sign to remember something you can pick which memory you want it to go to.

18:06

18 minutes, 6 seconds

So yeah that's the next step. take the time to configure quantumd servers all the stuff that your team uses so that

18:13

18 minutes, 13 seconds

you can use it once, configure it once and then share it with everyone.

18:19

18 minutes, 19 seconds

Um, an example of this is uh in our apps repo uh for anthropic. This is like the repo that we have all of our uh web and

18:26

18 minutes, 26 seconds

apps code in. There's a Puppeteer MCP server and we share this with the team um and there's a MCP JSON checked in. So any engineer working that repo can use

18:35

18 minutes, 35 seconds

Puppeteer in order to pilot end to end tests and to screenshot automatically and iterate so that every engineer doesn't have to install it themselves.

18:46

18 minutes, 46 seconds

This is a talk about pro tips. So I I just want to take a quick interlude to talk about some common key bindings that people may not know. It's a it's very hard to build for terminal. It's also

18:54

18 minutes, 54 seconds

very fun. It feels like rediscovering this new design language. But something about terminal is it's it's extremely minimal. And so sometimes it's hard to

19:02

19 minutes, 2 seconds

discover these key bindings. And here's just a quick reference sheet. So anytime you can hit shift tab to accept edits.

19:09

19 minutes, 9 seconds

Uh and this switches you into autoac accept edits mode. So bash commands still need approval, but edits are auto accepted. And you can always ask quad to undo them later.

19:18

19 minutes, 18 seconds

Um for example, I'll do this if I know Claude's on the right track or if it's writing unit tests and iterating on tests. I'll usually just switch into auto accept mode so I don't have to okay

19:26

19 minutes, 26 seconds

every single edit anytime you want Claude to remember something. So, for example, if it's not using a tool correctly and you wanted to

19:34

19 minutes, 34 seconds

use it correctly from then on, just type the pound sign and then tell it what to remember and it'll remember it. It'll incorporate it into QuadMD automatically.

19:42

19 minutes, 42 seconds

If you ever want to drop down to bash mode, so just run a bash command. You can hit the exclamation mark and type in your command. That'll run locally, but that also goes into the context window.

19:51

19 minutes, 51 seconds

So, Claude will see it on the next turn.

19:53

19 minutes, 53 seconds

Um, and this is pretty good for long running commands if you know exactly what you want to do or any command that you want to get into context and cloud will see the command and the output.

20:03

20 minutes, 3 seconds

You can appment mention files and folders. Uh, anytime you can hit escape to stop what Claude is doing. Um, no matter what Claude is doing, you can always safely hit escape. It's not going

20:11

20 minutes, 11 seconds

to corrupt the session. It's not going to mess anything up. So maybe Claude is doing a file edit. I'll hit escape. I'll tell it what to do differently. Or maybe

20:18

20 minutes, 18 seconds

it suggested a 20 line edit and I'm like actually 19 of these lines look perfect but one line you should change. I'll hit escape. I'll tell it that and then I'll tell it to redo the edit.

20:28

20 minutes, 28 seconds

Uh you can hit escape twice to jump back in history. Um and then after you're done with the session you can start quad with a resume to resume that session if

20:36

20 minutes, 36 seconds

you want. um or d- continue and then anytime if you want to see more output hit control-R and that'll show

20:44

20 minutes, 44 seconds

you the entire output the same thing that claude sees in its context window.

20:53

20 minutes, 53 seconds

The next thing I want to talk about is the cloud code SDK. So we talked about this at the top uh right after this Sid is doing a session I think just across

21:01

21 minutes, 1 second

the hallway and he's going to go super deep on the SDK. If you hadn't played around with this, if you used the -p flag in Claude, this is what the SDK is.

21:10

21 minutes, 10 seconds

And we've been learning a bunch of features over the last few weeks to make it even even better.

21:14

21 minutes, 14 seconds

Um, so yeah, you can you can build on top of this. You can do cool stuff. This is exactly the thing that cla code uses. It's exactly the same SDK.

21:22

21 minutes, 22 seconds

And so, for example, something you can do is cla. So, this is the the CLI SDK. You can pass a you can pass a prompt.

21:30

21 minutes, 30 seconds

You can pass some allowed tools which could include specific batch commands and you can tell it which format you want. So you might want JSON or you

21:37

21 minutes, 37 seconds

might want streaming JSON if you want to process this somehow. So this is awesome for for building on. We use this in CI all the time. We use this for incident

21:46

21 minutes, 46 seconds

response. We use this in all sorts of pipelines. So really convenient. Just think of it as like a Unix utility. You give it a prompt. It gives you JSON. You can use this in any way. You can pipe into it. You can pipe out of it.

22:00

22 minutes

The piping is also pretty cool. So you can use like for example git status and pipe this in and you know use jq to select the result. It the combinations

22:09

22 minutes, 9 seconds

are endless and it's sort of this new idea. It's like a super intelligent Unix utility and I think we barely scratched the surface of how to use this. We're just figuring this out.

22:18

22 minutes, 18 seconds

You can read from like a GCP bucket read, you know, like a giant log and pipe it in and tell cloud to figure out what's interesting about this log. Um,

22:26

22 minutes, 26 seconds

you can fetch data from like the Sentry uh CLI. You can also pipe it in and have Claude do something with it.

22:38

22 minutes, 38 seconds

The final thing, and this is probably like the most advanced use cases we see.

22:41

22 minutes, 41 seconds

I'm sort of a cloud normie, so I'll have usually like one cloud running at a time and maybe I'll have like a few terminal tabs for a few different repos running

22:49

22 minutes, 49 seconds

at a time. When I look at power users in an adavanthropic, almost always they're going to have like SSH sessions. They'll have uh like T-Max tunnels into their quad sessions.

22:59

22 minutes, 59 seconds

They're going to have a bunch of checkouts of the same repo so that they can run a bunch of quads in parallel in that repo or they're using git work trees to have some kind of isolation as

23:07

23 minutes, 7 seconds

they do this. And we're actively working on making this easier to use. But uh for now, like these these are some ideas for

23:14

23 minutes, 14 seconds

how to do more work in parallel with quad. You can run as many sessions as you want.

23:19

23 minutes, 19 seconds

Uh, and there's a lot that you can get done in perl.

23:25

23 minutes, 25 seconds

So, yeah, that's it. I wanted to also leave some time for Q&A. So, I think this is the last slide that I have. And yeah, if folks have questions, there's mics on both sides.

23:35

23 minutes, 35 seconds

And yeah, we'd love to answer any questions.

23:40

23 minutes, 40 seconds

[applause]

23:52

23 minutes, 52 seconds

[laughter]

23:54

23 minutes, 54 seconds

I did.

23:57

23 minutes, 57 seconds

[laughter]

24:00

24 minutes

Hey Boris, thanks for building a cloud code. Um, I was wondering what was the hardest implementation like part of the implementation for you of building it?

24:11

24 minutes, 11 seconds

I think there's a lot of tricky parts.

24:14

24 minutes, 14 seconds

Um I think one part that is especially tricky is the things that we do to make bash commands safe. Um bash is

24:22

24 minutes, 22 seconds

inherently pretty dangerous and it can change system state in unexpected ways.

24:26

24 minutes, 26 seconds

But at the same time, if you have to manually approve every single bash command, it's super annoying as an engineer and you can't really be

24:34

24 minutes, 34 seconds

productive because you're just constantly approving every command. and just kind of navigating how to do this safely in a way that that scales across the different kinds of code bases people

24:42

24 minutes, 42 seconds

have because not everyone runs their code in a Docker container um was was pretty tricky and essentially the thing we landed on is there's some commands

24:50

24 minutes, 50 seconds

that are readonly there's some static analysis that we do in order to figure out which commands can be combined in safe ways and then we have this pretty

24:57

24 minutes, 57 seconds

complex tiered permission system so that you can allow list and block list commands at different levels.

25:05

25 minutes, 5 seconds

Hi Boris, you mentioned uh giving an image to cloud code which made me wonder if there's some sort of multimodal functionality that I'm not aware of. Is

25:14

25 minutes, 14 seconds

that are you just pointing it at an image on the file system or something? Yeah, so cloud code is fully multimodal.

25:19

25 minutes, 19 seconds

Um it has been from the start it's in a terminal so it's a little uh hard to discover. Uh but yeah, you can take an image and just drag and drop it in

25:26

25 minutes, 26 seconds

that'll work. Uh you can give it a file path that'll work. Um you can copy and paste the image in and that works too.

25:35

25 minutes, 35 seconds

Um, so I'll use this pretty often for if I have like a mock of something, I'll just drag and drop drop in the mock.

25:39

25 minutes, 39 seconds

I'll tell it to implement it. I'll give it up a tier server so it can iterate against it. And yeah, it's just fully automated.

25:48

25 minutes, 48 seconds

Um, hey, uh, why did you build a CLI tool instead of an IDE?

25:54

25 minutes, 54 seconds

Yeah, it's it's a good question. I think there's probably two reasons. One is uh we started this at anthropic and atropic people use a broad range of ids and some

26:03

26 minutes, 3 seconds

people use VS code other people use zed or xcode or vim or emacs and it was just hard to build something that works for

26:11

26 minutes, 11 seconds

everyone and so terminal is just the common denominator. The second thing is at anthropic we're uh we see up close

26:20

26 minutes, 20 seconds

how fast the model is getting better and so I think there's a good chance that by the end of the year people aren't using IDs anymore.

26:27

26 minutes, 27 seconds

And so we want to get ready for this future and we want to avoid overinvesting in UI and other layers on top given that the way the models are

26:35

26 minutes, 35 seconds

progressing it just it may not be useful work pretty soon.

26:42

26 minutes, 42 seconds

Yeah. How much of you I don't know if this is is this on how much you used code for machine learning modeling and

26:50

26 minutes, 50 seconds

almost that autoML experience. I was curious what the experience has been so far with that. Yeah, I think I think the question was how much are we using cloud

26:59

26 minutes, 59 seconds

code for machine learning and and modeling? We actually use it for this a bunch. So both engineers and researchers at Enthropic use quad code every day. Um

27:08

27 minutes, 8 seconds

I think about 80% of people at Anthropic that are technical use quad code every day and hopefully you can see that in the product and kind of the amount of love and dog fooding we've put into it.

27:18

27 minutes, 18 seconds

Um but this includes researchers who use tools like the notebook notebook tool to edit and run notebooks. Okay, very cool. Thank you.

27:27

27 minutes, 27 seconds

All right, I think that's it. [applause] Thanks,
