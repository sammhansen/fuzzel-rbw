#!/usr/bin/env bash

# Same implementation in a bash script

# Initital handshake
echo "OK"  

while read cmd rest; do
    case "$cmd" in
        GETPIN|getpin)
            password=$(printf "" | fuzzel --prompt-only "${rest}" --password --dmenu)

            if [ -z "$password" ]; then
                # user cancelled
                echo "ERR 83886179 User canceled"
            else
                # send password back to gpg
                echo "D $password"
                echo "OK"
            fi
            ;;

        BYE|bye)
            echo "OK"
            exit 0
            ;;

        *)
            # For anything else, just respond OK
            echo "OK"
            ;;
    esac
done

