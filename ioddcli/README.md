

## Read Current values from Device
cargo run eval --ip 192.168.56.89 --port 1
## Read Device variable by index
RUST_LOG=info cargo run read-parameter 192.168.56.89 2 551 0 

## List all Variables
RUST_LOG=info cargo run  list-variable --vendorid 310 --deviceid 706 --productname TN2405 --accessrights rw

## Show variable by Name (id)
RUST_LOG=info cargo run  variable --vendorid 310 --deviceid 706 --productname TN2405 --id V_diS

## Read Parameter from IO DEVICE
cli read-parameter --ip 192.168.56.89 --port 2 --index 551 --subindex 0

## Encode Variable
RUST_LOG=info cargo run  encode-variable --vendorid 310 --deviceid 706 --productname TN2405 --id V_diS --param false, true, 4

## Set - Parameter 
cargo run set-parameter --ip 192.168.56.89 --port 2 --index 552 --subindex 0 --hexdata 4400

## Convert RawValues to Hex
RUST_LOG=info cargo run  encode-variable --vendorid 310 --deviceid 706 --productname TN2405 --id V_uni --param "0"

or 
RUST_LOG=info cargo run  encode-variable --vendorid 310 --deviceid 706 --productname TN2405 --id V_diS --param "true, false, 4" 


## Read Menus
cargo run get-menu  --vendorid 310 --deviceid 706 --productname TN2405 --role observer --menu parameter 