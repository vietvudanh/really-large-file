#!/usr/bin/env bash
CWD="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# DATA_FILE=$CWD/data/itcont.txt
# NO LONGER EXISTS
# DATA_FILE=$CWD/data/by_date/itcont_2018_invalid_dates.txt
DATA_FILE=~/data/misc/itcont.txt
LOG_FILE=log.log

# build
echo "run build"
bash $CWD/build.sh

# 
chmod -R +x dist

echo
echo "==== RUN ===="
echo "list of script"
ls "$CWD/dist"

echo
# all exe file will receive file from args
date >> $LOG_FILE
for exe in "$CWD/dist/"* ; do
  echo
	echo "running:: $exe" | tee -a $LOG_FILE
  case "$exe" in
    *.scala)
      time scala "$exe" "$DATA_FILE" | tee -a $LOG_FILE
      ;;
    *)
      time "$exe" "$DATA_FILE" | tee -a $LOG_FILE
      ;;
  esac
done
