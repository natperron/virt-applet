# virt-applet
Disclaimer: This is far from functional, but feel free to contribute! Also, I'm a **huge** beginner in Rust

This is meant to be an applet for libvirt. You should see a list of your virtual machines using libvirt.

How to install:
```SH
git clone https://github.com/natperron/virt-applet
cargo build --release
cd target/release
chmod +x ./virt-applet
```

If you want to add it to your PATH:
```SH
sudo mv ./target/release/virt-applet /usr/local/bin
```

Finally, how to run:
```SH
./virt-applet
# Or better, if you have it in your PATH
virt-applet
```

TODO: 
- Make a function to start a selected VM
- Make a function to stop a selected VM
- Improve code splitting
- Add a list of dependencies people need on their distro in order to run it on their machine (without needing cargo)
- Add github actions to build packages (.deb, .rpm, AUR) automatically
- Use a config file:
    - Setup pre/post scripts to run when starting/stopping a VM
    - Change the connection
    - Probably accomodate for a bunch of stuff I didn't think
