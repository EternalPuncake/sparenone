## Spare None

Sparenone is a tool for interacting with your files. 

* Capable of deleting folders
* Deletes a specific file extension in a folder
* Searches through a drive or folder for a specified file
* Copies files from one place in to another

---

   ![Image](/assets/video.gif)

---


Contents
========
   * [Why?](#why)
   * [Install](#install)
   * [Usage](#usage)


## why
   <p>I originally wanted a tool that mass deletes files, but now thats changed</p>
   
   <p>Now i have a tool that does many things</p>

   * delete files 
   * copy files
   * search for a file based on the name
   * look for a specific extension and delete those files

   Hopefully `sparenone` is of use to you

*** 

## Install
   > **NOTE:**
   > You should not need admin privileges for this

   ### Method 1: 
   ```
   git clone https://github.com/EternalPuncake/sparenone.git
   ```
   And then build with [`cargo`](https://doc.rust-lang.org/cargo/commands/cargo-run.html)

   <p>Now add it to your system path</p>

   ### Method 2:
   get from the releases tab

   ***

## usage

   * Run `sparenone`
   * Enter a [Command](#commands)
   * Follow the instructions
   * success >:(
   

## commands
   > currently there are only 5 commands

   * file-delete
   > will delete a file or a folder and all of the content inside
   * delete-many
   > will delete all files given
   * remove-suffix
   > will look throughout a folder and find and remove a certain file extension
   * look-for
   > Searches a drive or folder looking for a file
   * copy
   > copies file a to point b
