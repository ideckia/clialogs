btype="debug"

# ./target/$btype/clialogs notification --title "title" --text "text"
# ./target/$btype/clialogs file-dialog
# ./target/$btype/clialogs message-dialog --title "title" --text "text"
# ./target/$btype/clialogs progress
# ./target/$btype/clialogs input --title "title" --label "label" --hint "placeholder"
# ./target/$btype/clialogs log-in
# ./target/$btype/clialogs calendar
# ./target/$btype/clialogs color
# ./target/$btype/clialogs list --value bat --value two --value trois --value cuatro
# ./target/$btype/clialogs select --option bat --option two --option trois --option cuatro
./target/$btype/clialogs custom --layout-path $PWD/custom_dialog.json