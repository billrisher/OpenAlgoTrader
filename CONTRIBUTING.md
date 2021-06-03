# Contributing to OpenAlgoTrader

Thank you for your interest in contributing to this project! Any changes are welcome, I just ask that you follow some common conventions.

---
# How to Contribute

OAT follows the standard Fork & Pull Request workflow for contribution. This means that you will fork the upstream repository, make a branch in your fork and then once finalized, open a pull request into the upstream fork's `master` branch.

If you'd like additional information on this workflow, [check out Chaser324's guide.](https://gist.github.com/Chaser324/ce0505fbed06b947d962)

## Bug Reporting

It's obviously very important for us to catch bugs very early during development, especially when dealing with money and real-time transactions. In order to report a bug, please open up an issue with the following format: 

>[provide general introduction to the issue logging and why it is relevant to this repository]
>
> ## Context
>
> [provide more detailed introduction to the issue itself and why it is relevant]
> 
> ## Process
>
> [ordered list the process to finding and recreating the issue, example below]
> 
> 1. User goes to delete a dataset (to save space or whatever)
> 2. User gets popup modal warning
> 3. User deletes and it's lost forever
> 
> ## Expected result
> 
> [describe what you would expect to have resulted from this process]
> 
> ## Current result
> 
> [describe what you you currently experience from this process, and thereby explain the bug]
>
> ## Possible Fix
>
> [not obligatory, but suggest fixes or reasons for the bug]
>
> * Modal tells the user what dataset is being deleted, like “You are about to delete this dataset: car_crashes_2014.”
> * A temporary "Trashcan" where you can recover a just deleted dataset if you mess up (maybe it's only good for a few hours, and then it cleans the cache assuming you made the right decision).
> 
> ## `name of issue` screenshot
>
> [if relevant, include a screenshot]

<div style="font-size:12px;">bug reporting template taken from <a href="https://gist.github.com/auremoser/72803ba969d0e61ff070">auremoser</a></div>

---

## Suggesting Enhancements

The addition of new features and the enhancement of existing features is always welcome. If you have a change that you wish to see or a feature that you want included in OAT, open up an issue similarly to how you would for a bug report. Make sure to include **what, why, who and how**:

- What is the enhancement you're suggesting used for?
- Why is it necessary for the enhancement to be included?
- Who would benefit from this enhancement?
- How would you go about implementing this enhancement?

Feel free to be as verbose as necessary, although the above bullets are the **bare minimum**.