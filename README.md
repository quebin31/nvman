# nvman

:warning: **WARNING** :warning: This script depends on `optimus-manager` which is only tested in
Arch Linux based distros.
## Introduction

This tiny script pretends to be a manager for both well known programs:
[`bumblebee`](https://github.com/Bumblebee-Project/Bumblebee) and
[`optimus-manager`](https://github.com/Askannz/optimus-manager). Both of them provide
solutions for laptops that have NVIDIA.

## What's the problem trying to solve?
If you already known how `bumblebee` and `optimus-manager` works, you may have noticed
that you may need both of them.

- Sometimes you want to run a program with `bumblebee` regarding the performance loss.
- Sometimes you want to run a program with `optimus-manager` regarding that you have to
    log out and log in.

You will have to stop and start your services manually all the time, also you need to keep
track of which of them is starting at boot, they are indeed conflictive.


## Dependencies 
- bumblebee (Official repo)
- primus (Official repo)
- optimus-manager (AUR)

## Installation
You should definetly take advantage of the `PKGBUILD` available on the AUR.

- **Using `yay`**
    ```
    yay -S nvman
    ```

- **Using `makepkg`**
    ```
    git clone https://aur.archlinux.org/nvman.git
    cd nvman
    makepkg -sic
    ```

## Configuration
There is a simple config file and it's located at `/etc/nvman/config`. The only valid
value is the default service started at boot. You won't need to modify this file manually
almost never, instead you should use `nvman default <bumblebee|optimus>`.


- **Config syntax:**
    ```
    default = <bumblebee|optimus>
    ```

- **Default config:**
    ```
    default = optimus
    ```

## Usage 

```
Commands:
  nvidia  <on|off>                Turn on/off NVIDIA GPU, useful for CUDA
  run     <cmd>                   Run any command with primusrun
  switch  <nvidia|intel|auto>     Switch gpu using optimus-manager
  startup <nvidia|intel>          Set startup gpu for optimus service
  default <bumblebee|optimus>     Set default service at boot
  start   <bumblebee|optimus>     Manually start the service
  stop    <bumblebee|optimus>     Manually stop the service

  help                            Show this help
  status                          Show the current status of both services
```

#### Commands examples

- Using `nvidia` to turn on/off your NVIDIA GPU on demand. This command is really useful
    for CUDA, and if you suspect that your NVIDIA GPU is on.
    ```
    nvman nvidia on
    ./cuda-program
    ```

- Using `run` to run something using `primusrun`
    ```
    nvman run glxgears
    ```

- Using `switch` to switch GPU using `optimus-manager`
    ```
    nvman switch auto
    ```

- Using `default` to set your default service at boot (default value: optimus)
    ```
    nvman default optimus
    ```

- Using `startup` to set your initial GPU on boot (only valid if your default service is
    optimus)

	```
    nvman startup intel
	```

- Using `status` to see what are you currently using (you can also just type `nvman`)
    ```
    Optimus          : active    (enabled)
    Bumblebee        : inactive  (disabled)
    NVIDIA GPU       : off
    Default service  : optimus
    Optimus mode     : intel
    Optimus startup  : intel
    ```

## F.A.Q.

> Why should I use `nvman`?

As I stated above, maybe you want to have `bumblebee` and `optimus-manager` on your
system. `nvman` will help you to work with them, without systemd headaches.

> What's doing `nvman.service`? Is it necessary?

It's TOTALLY necessary to have enabled `nvman.service`, it basically takes care that only
one of the services (`bumblebee` or `optimus`) start at boot, since they can break your
system if both of them are enabled.

