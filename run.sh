#!/usr/bin/env bash
CWD="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
DATA_FILE=$CWD/data/itcont.txt
# DATA_FILE=$CWD/data/by_date/itcont_2018_invalid_dates.txt
LOG_FILE=log.log

# build
echo "run build"
bash $CWD/build.sh

# 
chmod -R +x dist

echo "list of script"
ls $CWD/dist

# all exe file will receive file from args
date >> $LOG_FILE
for exe in $(ls $CWD/dist/) ; do
    echo
	echo "running:: $exe" | tee -a $LOG_FILE
	/usr/bin/time -f "real time::%Es" $CWD/dist/$exe $DATA_FILE | tee -a $LOG_FILE
done
