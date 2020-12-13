#!/bin/sh
for D in day??
do
    echo $D
    ( cd $D
      PARTS="1 2"
      if [ $D = day25 ]
      then
          PARTS="1"
      fi
      for PART in $PARTS
      do
          if ! [ -x target/release/soln ]
          then
              echo "  build"
              cargo build --release >/dev/null 2>&1
          fi
          if [ "$PARTS" = "1" ]
          then
              echo -n "  sole $PART: "
              CMD="`egrep \"^     *cargo run --release\" README.md`"
          else
              echo -n "  part $PART: "
              CMD="`egrep \"^     *cargo run --release $PART\" README.md`"
          fi
          if [ $? -ne 0 ]
          then
              echo "  part $PART: could not find command" >&2
              continue
          fi
          CMD="`echo \"$CMD\" | sed 's= *cargo run --release=/usr/bin/time -f '%e' target/release/soln='`"
          sh -c "$CMD 2>&1 >/dev/null" 
      done )
done
