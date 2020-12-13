#!/bin/sh
for D in day??
do
    ( cd $D && cargo clippy )
done
if [ "$1" = '-a' ]
then
    ( cd libaoc && cargo clippy )
fi
