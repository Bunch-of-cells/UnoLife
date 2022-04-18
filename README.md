<br>

<div align="center">
  <img src="assets/unolife_logo.png" width="340" height="340" alt="logo">

  <br>
  <br>

[![Generic badge](https://img.shields.io/badge/CodeJam-Timathon-orange.svg)](https://twtcodejam.net/) ![Platforms](https://raster.shields.io/badge/Platform-All-blue.png) ![GitHub repo size](https://img.shields.io/github/repo-size/Bunch-of-cells/UnoLife)

</div>

# UnoLife, the way of unordinary life

This is the perfect app for anyone looking to spice up their life! It has a bunch of apps and features
to help you make your everyday less ordinary by keeping you occupied with new and exiting games!

You can try the app by downloading one of the prebuilt releases or building it from scratch using rust.
It should work on all platforms, please don't hesitate to contact us and look at the faq if you have any issues or questions.

## The idea

The original idea was to make an app similar to Dekstop Goose where a goose is interuppting your everyday use.
Here is a video showcasing that application: https://www.youtube.com/watch?v=EQx6fyrZDWM

However due to complications with the UI library we were using and the lack of time we decided to settle on a more standard approach of a minigame collection.

## Installation and Setup

The app is already prebuilt for windows and linux so all you have to do is go to the releases, download the zip file and then run the binary after unzipping.

If you are using mac or want to build from scratch you have to first download rust, which can be done from the offical site: https://www.rust-lang.org/tools/install
</br>

For linux users: You have to install a couple of dependencies if building on linux. These are libssl-dev and pkg-config.
On ubuntu and other similar distros these can be installed in one command with: `sudo apt install libssl-dev pkg-config`
<br>
However this might be different for other distros so please check the correct package names for your distro if the above command does not work.

After that just clone the repo, cd into the project folder and type `cargo run`. It should start building the project and after a few minutes it should start the application and you can start using it.

NOTE: If you are building the binary from scratch, please run the binary from the root folder as it might not find the assets folder otherwise. If you are using `cargo run` it should automatically be ran from the correct folder so you don't have to worry about this.

## Features

- TicTacToe: Play the classic game of tictactoe against a friend or AI that we made!
- Wordle: Play the ever popular wordle game remastered using rust in our app!
- Snake: Play the classic snake game and try to get a highscore!
- 2048: Play 2058 while enjoying the nice looking graphics!
- 15 Puzzle: Play fifteen puzzle the classic way!
- Reddit meme: Get a random reddit meme from popular subreddits and view them right in the app!
- Highscores: The app supports highscores for every game, so you will always have something to look forward to beating when playing!
- Settings: You can customise the look and feel of the app by for example changing the theme!

## Contributions

We love your open source enthusiasm. Seeing a application grow a bigger community is possibly the best thing a developer can expect.
We don't accept pull requests for now as the codejam is still ongoing, but we will update the README once it ends and feel free to make a pull request then.
